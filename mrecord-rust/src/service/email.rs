//! 邮件服务
//!
//! 对应 Java: `com.dcz.mrecord.service.EmailService`
//! 实现:    `com.dcz.mrecord.service.impl.EmailServiceImpl`
//!
//! 设计要点：
//! - SMTP 客户端使用 [`lettre`]（与 Java 端 `apache-commons-email/HtmlEmail` 等价）。
//! - SMTP 配置从 [`crate::service::sys_config::SysConfigService`] 读取，
//!   未配置或不完整时返回业务错误，与 Java 行为一致。
//! - 模板使用 `include_str!` 在编译期嵌入到二进制，运行时不再读盘；
//!   占位符语法保持与 Java 一致：`${MR-XXX}`。
//! - 发送顺序：连接 → 认证 → 发送。出错统一封装为 `AppError::Business`，
//!   并在 `tracing::error!` 中记录详细原因。
//!
//! 注意：Java 版本中 `getMailClient` 强制 `setSSLOnConnect(true)`、`setStartTLSEnabled(false)`，
//! 即仅支持 SSL/SMTPS（隐式 TLS）。Rust 端按相同策略：使用 `SmtpTransport::relay`
//! 的 SMTPS 连接器，端口取 `mail.sslSmtpPort`。

use std::{collections::HashMap, path::Path, sync::Arc};

use lettre::{
    message::{header::ContentType, Attachment, Mailbox, MultiPart, SinglePart}, transport::smtp::{authentication::Credentials, client::Tls, client::TlsParameters}, AsyncSmtpTransport, AsyncTransport,
    Message,
    Tokio1Executor,
};
use sea_orm::DatabaseConnection;

use crate::{
    common::res_code::ResCode,
    error::AppError,
    model::{email::EmailConfigBo, mail_params::MailParams},
    service::sys_config::SysConfigService,
};

// ==================== 内嵌邮件模板 ====================
//
// 与 Java 端 `src/main/resources/mail/*.html` 一一对应，编译期嵌入二进制。

/// 账户激活邮件模板（对应 Java `mail/mr-activate.html`）
const TEMPLATE_ACTIVATE: &str = include_str!("../../resources/mail/mr-activate.html");
/// 注册成功欢迎邮件模板（对应 Java `mail/mr-register.html`）
const TEMPLATE_REGISTER: &str = include_str!("../../resources/mail/mr-register.html");
/// 找回密码邮件模板（对应 Java `mail/mr-password.html`）
const TEMPLATE_PASSWORD: &str = include_str!("../../resources/mail/mr-password.html");
/// 月度记账提醒模板（对应 Java `mail/mr-bookkeep.html`）
#[allow(dead_code)]
const TEMPLATE_BOOKKEEP: &str = include_str!("../../resources/mail/mr-bookkeep.html");
/// 新财年总结模板（对应 Java `mail/mr-year.html`）
#[allow(dead_code)]
const TEMPLATE_YEAR: &str = include_str!("../../resources/mail/mr-year.html");
/// 账簿导出完成模板（对应 Java `mail/mr-export.html`）
#[allow(dead_code)]
const TEMPLATE_EXPORT: &str = include_str!("../../resources/mail/mr-export.html");

/// 邮件服务
///
/// 通过 `Arc` 在多个 handler 间共享。本身无状态，依赖 [`SysConfigService`] 读取 SMTP 配置。
pub struct EmailService {
    config_service: Arc<SysConfigService>,
}

impl EmailService {
    /// 创建一个新的服务实例
    pub fn new(config_service: Arc<SysConfigService>) -> Arc<Self> {
        Arc::new(Self { config_service })
    }

    // ==================== 公开方法 ====================

    /// 发送账户激活邮件
    ///
    /// 对应 Java: `EmailServiceImpl.sendActivateAccountEmail`
    pub async fn send_activate_account_email(
        &self,
        db: &DatabaseConnection,
        params: MailParams,
    ) -> Result<(), AppError> {
        self.send_with_db_config(
            db,
            &params,
            "【MRecord｜月衡】账户激活",
            TEMPLATE_ACTIVATE,
            "账户激活邮件发送失败",
            true,
        )
            .await
    }

    /// 发送注册成功欢迎邮件
    ///
    /// 对应 Java: `EmailServiceImpl.sendRegisterSuccessEmail`。
    /// Java 端这条邮件失败仅记日志、不抛异常；Rust 端保持相同策略。
    pub async fn send_register_success_email(&self, db: &DatabaseConnection, params: MailParams) {
        if let Err(e) = self
            .send_with_db_config(
                db,
                &params,
                "【MRecord｜月衡】欢迎使用月衡",
                TEMPLATE_REGISTER,
                "注册欢迎邮件发送失败",
                false,
            )
            .await
        {
            tracing::warn!("注册欢迎邮件发送失败: {:?}", e);
        }
    }

    /// 发送找回密码邮件
    ///
    /// 对应 Java: `EmailServiceImpl.sendRetrievePasswordEmail`。
    pub async fn send_retrieve_password_email(
        &self,
        db: &DatabaseConnection,
        params: MailParams,
    ) -> Result<(), AppError> {
        self.send_with_db_config(
            db,
            &params,
            "【MRecord｜月衡】密码重置",
            TEMPLATE_PASSWORD,
            "重置密码邮件发送失败",
            true,
        )
            .await
    }

    /// 发送账簿导出完成邮件。
    ///
    /// 对应 Java: `EmailServiceImpl.sendExportBookEmail`。
    pub async fn send_export_book_email(
        &self,
        db: &DatabaseConnection,
        params: MailParams,
        attachment_path: &Path,
    ) -> Result<(), AppError> {
        self.send_with_db_config_and_attachment(
            db,
            &params,
            "【MRecord｜月衡】账簿导出完成",
            TEMPLATE_EXPORT,
            "账簿导出完成邮件发送失败",
            attachment_path,
        )
            .await
    }

    /// 发送测试邮件（使用前端传入的临时配置，不读 DB）
    ///
    /// 对应 Java: `EmailServiceImpl.sendTestEmail`
    pub async fn send_test_email(&self, config: &EmailConfigBo, to: &str) -> Result<(), AppError> {
        let body =
            "<p>这是一封来自 <b>月衡 Mrecord</b> 的测试邮件，收到此邮件说明邮箱配置正确。</p>";
        let message = build_message(config, to, "月衡 Mrecord - 邮件配置测试", body)?;
        let transport = build_transport(config)?;
        transport.send(message).await.map_err(|e| {
            tracing::error!("发送测试邮件失败: {:?}", e);
            AppError::Business {
                code: ResCode::ParamError.code().to_string(),
                message: format!("邮件发送失败：{}", e),
            }
        })?;
        Ok(())
    }

    // ==================== 内部辅助 ====================

    /// 通用发送：从 DB 读取邮件配置 + 站点信息，渲染模板，发送 HTML 邮件。
    ///
    /// `mandatory` 参数语义对齐 Java：
    /// - `true`：管理员未配置邮件 → 抛 `BusinessError`（用户可见的提示）；
    /// - `false`：缺配置时仅记 warn 后跳过（Java 端 `sendRegisterSuccessEmail` 即此分支）。
    async fn send_with_db_config(
        &self,
        db: &DatabaseConnection,
        params: &MailParams,
        subject: &str,
        template: &str,
        err_hint: &str,
        mandatory: bool,
    ) -> Result<(), AppError> {
        let Some(mail_cfg) = self.load_mail_config(db, err_hint, mandatory).await? else {
            tracing::warn!("管理员未配置邮件参数，跳过邮件发送：{}", subject);
            return Ok(());
        };
        let html = self.render_with_site_config(db, params, template).await?;

        let message = build_message(&mail_cfg, &params.to, subject, &html)?;
        let transport = build_transport(&mail_cfg)?;
        transport.send(message).await.map_err(|e| {
            tracing::error!("{}: {:?}", err_hint, e);
            AppError::Business {
                code: ResCode::Error.code().to_string(),
                message: format!("{}，请联系管理员", err_hint),
            }
        })?;
        Ok(())
    }

    /// 通用发送：从 DB 读取邮件配置并添加附件。
    ///
    /// 对应 Java `HtmlEmail.attach` 的导出文件附件场景。
    async fn send_with_db_config_and_attachment(
        &self,
        db: &DatabaseConnection,
        params: &MailParams,
        subject: &str,
        template: &str,
        err_hint: &str,
        attachment_path: &Path,
    ) -> Result<(), AppError> {
        let Some(mail_cfg) = self.load_mail_config(db, err_hint, true).await? else {
            return Ok(());
        };
        let html = self.render_with_site_config(db, params, template).await?;
        let bytes = std::fs::read(attachment_path).map_err(|e| AppError::Business {
            code: ResCode::Error.code().to_string(),
            message: format!("读取导出附件失败：{}", e),
        })?;
        let file_name = params.file_name.clone();
        let content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            .parse()
            .map_err(|e| AppError::Business {
                code: ResCode::Error.code().to_string(),
                message: format!("附件类型构造失败：{}", e),
            })?;
        let body = MultiPart::mixed()
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html),
            )
            .singlepart(Attachment::new(file_name).body(bytes, content_type));
        let message = build_multipart_message(&mail_cfg, &params.to, subject, body)?;
        let transport = build_transport(&mail_cfg)?;
        transport.send(message).await.map_err(|e| {
            tracing::error!("{}: {:?}", err_hint, e);
            AppError::Business {
                code: ResCode::Error.code().to_string(),
                message: format!("{}，请联系管理员", err_hint),
            }
        })?;
        Ok(())
    }

    /// 读取 SMTP 配置，保留 Java 中强制/非强制发送的差异。
    async fn load_mail_config(
        &self,
        db: &DatabaseConnection,
        err_hint: &str,
        mandatory: bool,
    ) -> Result<Option<EmailConfigBo>, AppError> {
        match self.config_service.get_email_config(db).await? {
            Some(cfg) => Ok(Some(cfg)),
            None => {
                if mandatory {
                    tracing::warn!("管理员未配置邮件参数，跳过 {}", err_hint);
                    Err(AppError::Business {
                        code: ResCode::BusinessError.code().to_string(),
                        message: "管理员未配置邮箱参数，请联系管理员".to_string(),
                    })
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// 填充站点地址、管理员邮箱等通用模板占位符。
    async fn render_with_site_config(
        &self,
        db: &DatabaseConnection,
        params: &MailParams,
        template: &str,
    ) -> Result<String, AppError> {
        let web_site = self.config_service.get_web_site(db).await?;
        let admin_mail = self.config_service.get_admin_mail(db).await?;
        let placeholders = params.to_placeholders(web_site.as_deref(), admin_mail.as_deref());
        Ok(render_template(template, &placeholders))
    }
}

// ==================== 模板渲染 ====================

/// 把 `${KEY}` 替换为 placeholders 中的对应值
///
/// 与 Java `EmailServiceImpl.readTemplate` 的字符串 `replace` 一致：直接逐 key 替换。
fn render_template(template: &str, placeholders: &HashMap<String, String>) -> String {
    let mut out = template.to_string();
    for (k, v) in placeholders {
        out = out.replace(&format!("${{{}}}", k), v);
    }
    out
}

// ==================== SMTP 构造 ====================

/// 构造一封 HTML 邮件
fn build_message(
    cfg: &EmailConfigBo,
    to: &str,
    subject: &str,
    html: &str,
) -> Result<Message, AppError> {
    base_message_builder(cfg, to, subject)?
        .header(ContentType::TEXT_HTML)
        .body(html.to_string())
        .map_err(|e| AppError::Business {
            code: ResCode::Error.code().to_string(),
            message: format!("邮件构建失败：{}", e),
        })
}

/// 构造一封带附件的多段邮件。
fn build_multipart_message(
    cfg: &EmailConfigBo,
    to: &str,
    subject: &str,
    body: MultiPart,
) -> Result<Message, AppError> {
    base_message_builder(cfg, to, subject)?
        .multipart(body)
        .map_err(|e| AppError::Business {
            code: ResCode::Error.code().to_string(),
            message: format!("邮件构建失败：{}", e),
        })
}

/// 构造邮件基础头信息。
fn base_message_builder(
    cfg: &EmailConfigBo,
    to: &str,
    subject: &str,
) -> Result<lettre::message::MessageBuilder, AppError> {
    let from: Mailbox = cfg.from.parse().map_err(|e| AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: format!("发件人邮箱格式错误：{}", e),
    })?;
    let to: Mailbox = to.parse().map_err(|e| AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: format!("收件人邮箱格式错误：{}", e),
    })?;

    Ok(Message::builder().from(from).to(to).subject(subject))
}

/// 构造 SMTP 客户端
///
/// 与 Java 端策略一致：始终走 SSL/SMTPS（隐式 TLS），端口取 `mail.sslSmtpPort`。
fn build_transport(cfg: &EmailConfigBo) -> Result<AsyncSmtpTransport<Tokio1Executor>, AppError> {
    let port = cfg.ssl_smtp_port.unwrap_or(465) as u16;

    let tls = TlsParameters::new(cfg.host_name.clone()).map_err(|e| AppError::Business {
        code: ResCode::Error.code().to_string(),
        message: format!("SMTP TLS 配置失败：{}", e),
    })?;

    let creds = Credentials::new(cfg.username.clone(), cfg.password.clone());

    let transport = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.host_name)
        .port(port)
        .tls(Tls::Wrapper(tls))
        .credentials(creds)
        .build();

    Ok(transport)
}
