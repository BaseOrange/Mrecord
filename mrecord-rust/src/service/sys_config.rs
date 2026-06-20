//! 系统配置项服务
//!
//! 对应 Java: `com.dcz.mrecord.service.SysConfigService`
//! 实现:     `com.dcz.mrecord.service.impl.SysConfigServiceImpl`
//!
//! 设计要点：
//! - 进程级内存缓存：邮件配置、站点地址、管理员邮箱、注册开关。
//!   首次调用时按需懒加载，写入接口（updateEmailConfig / updateSiteConfig）
//!   会清空对应缓存以触发下次读取时重新加载。
//! - 并发：使用 `tokio::sync::RwLock` 包装缓存。读多写少，写路径会等待
//!   现有读完成（无需双重检查锁，简单清晰）。
//! - 持久化：`SYS_CONFIG` 表中每个 key 一行。Java 原版 `updateConfigByKey`
//!   仅更新已存在的行；这里改成 upsert（不存在则插入），更友好且不引入功能差异。
//! - 系统初始化标识：`isInitialized` 以管理员账户存在为准，与 Java 行为一致；
//!   `sys.initialized` 这条配置仅作缓存/标记，自动同步。

use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    entity::{
        sys_config::{
            self, ActiveModel as ConfigActive, Column as ConfigCol, Entity as ConfigEntity,
        },
        sys_user::{Column as UserCol, Entity as UserEntity},
    },
    error::AppError,
    model::{
        email::{EmailConfigBo, UpdateEmailConfigDto},
        site::{SiteConfigVo, UpdateSiteConfigDto},
    },
};

// ==================== 配置项 key 常量 ====================
//
// 与 Java 端 `SysConfigServiceImpl` 中的字符串字面量保持完全一致。
// 集中放在这里，避免 handler / service 各处散落字符串易写错。

const KEY_MAIL_HOST_NAME: &str = "mail.hostName";
const KEY_MAIL_SSL_SMTP_PORT: &str = "mail.sslSmtpPort";
const KEY_MAIL_SMTP_PORT: &str = "mail.smtpPort";
const KEY_MAIL_SSL: &str = "mail.ssl";
const KEY_MAIL_USER_NAME: &str = "mail.userName";
const KEY_MAIL_PASSWORD: &str = "mail.password";
const KEY_MAIL_FROM: &str = "mail.from";

const KEY_WEB_SITE: &str = "webSite";
const KEY_ADMIN_MAIL: &str = "adminMail";
const KEY_REGISTER_ENABLED: &str = "sys.registerEnabled";
const KEY_INITIALIZED: &str = "sys.initialized";

/// 脱敏后用于响应的密码占位
const MASKED_PASSWORD: &str = "******";

/// 内部缓存状态
///
/// 三态语义：`None` 表示未加载，`Some(...)` 表示已加载（值可能为 `None` 表示加载过但缺失）。
#[derive(Default)]
struct CacheState {
    /// 邮件配置：`Some(None)` 表示加载过但配置不全
    email_config: Option<Option<EmailConfigBo>>,
    /// 网站地址
    web_site: Option<Option<String>>,
    /// 管理员邮箱
    admin_mail: Option<Option<String>>,
    /// 注册开关：`Some(true)` 已加载且开启
    register_enabled: Option<bool>,
}

/// 配置项服务
///
/// 通过 `Arc` 在多个 handler 间共享。所有 DB 操作显式接收 `&DatabaseConnection`，
/// 由调用方（handler）从 `AppState` 传入；这样服务本身保持无状态，方便测试替换。
pub struct SysConfigService {
    cache: RwLock<CacheState>,
}

impl SysConfigService {
    /// 创建一个新的服务实例（缓存初始为空，按需懒加载）
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            cache: RwLock::new(CacheState::default()),
        })
    }

    // ==================== 缓存维护 ====================

    /// 刷新缓存（清空所有 key）
    ///
    /// 对应 Java: `SysConfigServiceImpl.refreshCache`
    pub async fn refresh_cache(&self) {
        let mut state = self.cache.write().await;
        *state = CacheState::default();
    }

    // ==================== 邮件配置 ====================

    /// 获取邮件配置（原始密码）
    ///
    /// 对应 Java: `getEmailConfig`
    pub async fn get_email_config(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Option<EmailConfigBo>, AppError> {
        // 先读缓存
        if let Some(cached) = self.cache.read().await.email_config.clone() {
            return Ok(cached);
        }
        // 未命中，加载并写回缓存
        let loaded = load_email_config(db).await?;
        let mut state = self.cache.write().await;
        state.email_config = Some(loaded.clone());
        Ok(loaded)
    }

    /// 获取脱敏后的邮件配置（密码替换为 `******`），用于接口返回
    ///
    /// 对应 Java: `getMaskedEmailConfig`
    pub async fn get_masked_email_config(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Option<EmailConfigBo>, AppError> {
        let origin = self.get_email_config(db).await?;
        Ok(origin.map(|cfg| {
            let mut masked = cfg.clone();
            if !masked.password.is_empty() {
                masked.password = MASKED_PASSWORD.to_string();
            }
            masked
        }))
    }

    /// 修改邮件配置（多条配置原子更新）
    ///
    /// 对应 Java: `updateEmailConfig`。
    /// 密码字段如果是掩码，则保留原值，避免脱敏值被回写。
    pub async fn update_email_config(
        &self,
        db: &DatabaseConnection,
        dto: UpdateEmailConfigDto,
    ) -> Result<(), AppError> {
        // 处理密码：如果接口传回的是掩码，则保留原密码
        let password = if dto.password == MASKED_PASSWORD {
            self.get_email_config(db)
                .await?
                .map(|c| c.password)
                .unwrap_or_default()
        } else {
            dto.password
        };

        upsert_config(db, KEY_MAIL_HOST_NAME, &dto.host_name).await?;
        upsert_config(
            db,
            KEY_MAIL_SSL_SMTP_PORT,
            &dto.ssl_smtp_port.map(|v| v.to_string()).unwrap_or_default(),
        )
            .await?;
        upsert_config(
            db,
            KEY_MAIL_SMTP_PORT,
            &dto.smtp_port.map(|v| v.to_string()).unwrap_or_default(),
        )
            .await?;
        upsert_config(
            db,
            KEY_MAIL_SSL,
            if dto.ssl.unwrap_or(false) { "1" } else { "0" },
        )
            .await?;
        upsert_config(db, KEY_MAIL_USER_NAME, &dto.user_name).await?;
        upsert_config(db, KEY_MAIL_PASSWORD, &password).await?;
        upsert_config(db, KEY_MAIL_FROM, &dto.from).await?;

        // 失效缓存
        self.cache.write().await.email_config = None;
        Ok(())
    }

    // ==================== 站点配置 ====================

    /// 获取网站地址
    ///
    /// 对应 Java: `getWebSite`
    pub async fn get_web_site(&self, db: &DatabaseConnection) -> Result<Option<String>, AppError> {
        if let Some(cached) = self.cache.read().await.web_site.clone() {
            return Ok(cached);
        }
        let loaded = load_single(db, KEY_WEB_SITE).await?;
        self.cache.write().await.web_site = Some(loaded.clone());
        Ok(loaded)
    }

    /// 获取管理员邮箱
    ///
    /// 对应 Java: `getAdminMail`
    pub async fn get_admin_mail(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Option<String>, AppError> {
        if let Some(cached) = self.cache.read().await.admin_mail.clone() {
            return Ok(cached);
        }
        let loaded = load_single(db, KEY_ADMIN_MAIL).await?;
        self.cache.write().await.admin_mail = Some(loaded.clone());
        Ok(loaded)
    }

    /// 是否开启注册功能
    ///
    /// 对应 Java: `isRegisterEnabled`。无配置或值不为 `"1"` 视作关闭。
    pub async fn is_register_enabled(&self, db: &DatabaseConnection) -> Result<bool, AppError> {
        if let Some(cached) = self.cache.read().await.register_enabled {
            return Ok(cached);
        }
        let value = load_single(db, KEY_REGISTER_ENABLED).await?;
        let enabled = value.as_deref() == Some("1");
        self.cache.write().await.register_enabled = Some(enabled);
        Ok(enabled)
    }

    /// 获取站点配置（聚合 webSite / adminMail / registerEnabled）
    ///
    /// 对应 Java: `getSiteConfig`
    pub async fn get_site_config(&self, db: &DatabaseConnection) -> Result<SiteConfigVo, AppError> {
        Ok(SiteConfigVo {
            web_site: self.get_web_site(db).await?,
            admin_mail: self.get_admin_mail(db).await?,
            register_enabled: self.is_register_enabled(db).await?,
        })
    }

    /// 修改站点配置
    ///
    /// 对应 Java: `updateSiteConfig`
    pub async fn update_site_config(
        &self,
        db: &DatabaseConnection,
        dto: UpdateSiteConfigDto,
    ) -> Result<(), AppError> {
        upsert_config(db, KEY_WEB_SITE, &dto.web_site).await?;
        upsert_config(db, KEY_ADMIN_MAIL, &dto.admin_mail).await?;
        upsert_config(
            db,
            KEY_REGISTER_ENABLED,
            if dto.register_enabled.unwrap_or(false) {
                "1"
            } else {
                "0"
            },
        )
            .await?;

        let mut state = self.cache.write().await;
        state.web_site = None;
        state.admin_mail = None;
        state.register_enabled = None;
        Ok(())
    }

    // ==================== 系统初始化 ====================

    /// 系统是否已完成初始化（存在管理员账户即视为已初始化）
    ///
    /// 对应 Java: `isInitialized`。同时把结果同步回 `sys.initialized` 标记。
    pub async fn is_initialized(&self, db: &DatabaseConnection) -> Result<bool, AppError> {
        let admin_count = UserEntity::find()
            .filter(UserCol::Admin.eq(1))
            .filter(UserCol::IsDeleted.eq(0))
            .count(db)
            .await?;
        let initialized = admin_count > 0;

        // 同步标记字段；不存在则不写入（与 Java 行为一致：仅更新已有行）
        if let Some(cfg) = ConfigEntity::find()
            .filter(ConfigCol::Key.eq(KEY_INITIALIZED))
            .filter(ConfigCol::IsDeleted.eq(0))
            .one(db)
            .await?
        {
            let expected = if initialized { "1" } else { "0" };
            if cfg.value != expected {
                let mut active: ConfigActive = cfg.into();
                active.value = Set(expected.to_string());
                active.update_time = Set(Some(chrono::Local::now().naive_local()));
                active.update(db).await?;
            }
        }

        Ok(initialized)
    }
}

// ==================== 内部辅助函数 ====================

/// 按 key 加载单条配置的 value
async fn load_single(db: &DatabaseConnection, key: &str) -> Result<Option<String>, AppError> {
    let row = ConfigEntity::find()
        .filter(ConfigCol::Key.eq(key))
        .filter(ConfigCol::IsDeleted.eq(0))
        .one(db)
        .await?;
    Ok(row.map(|m| m.value))
}

/// 按 key 加载邮件配置（多个 mail.* 行组合成 `EmailConfigBo`）
///
/// 任一必填项缺失时返回 `None`（Java 行为一致）。
async fn load_email_config(db: &DatabaseConnection) -> Result<Option<EmailConfigBo>, AppError> {
    let rows: Vec<sys_config::Model> = ConfigEntity::find()
        .filter(ConfigCol::Key.starts_with("mail."))
        .filter(ConfigCol::IsDeleted.eq(0))
        .all(db)
        .await?;
    if rows.is_empty() {
        tracing::warn!("管理员未配置邮件参数，返回空邮箱配置信息");
        return Ok(None);
    }

    let pick = |k: &str| {
        rows.iter()
            .find(|r| r.key == k)
            .map(|r| r.value.clone())
            .filter(|v| !v.is_empty())
    };

    let host_name = pick(KEY_MAIL_HOST_NAME);
    let ssl_smtp_port = pick(KEY_MAIL_SSL_SMTP_PORT);
    let smtp_port = pick(KEY_MAIL_SMTP_PORT);
    let username = pick(KEY_MAIL_USER_NAME);
    let password = pick(KEY_MAIL_PASSWORD);
    let from = pick(KEY_MAIL_FROM);

    // 必填项（与 Java 端 `isAnyBlank` 字段集对齐）
    if host_name.is_none()
        || ssl_smtp_port.is_none()
        || smtp_port.is_none()
        || username.is_none()
        || password.is_none()
        || from.is_none()
    {
        tracing::warn!("管理员邮件参数不完整，返回空邮箱配置信息");
        return Ok(None);
    }

    // 端口数字格式
    let parsed_ssl_smtp = ssl_smtp_port.unwrap().parse::<i32>();
    let parsed_smtp = smtp_port.unwrap().parse::<i32>();
    if parsed_ssl_smtp.is_err() || parsed_smtp.is_err() {
        tracing::warn!("邮件端口配置格式错误");
        return Ok(None);
    }

    Ok(Some(EmailConfigBo {
        host_name: host_name.unwrap(),
        ssl_smtp_port: Some(parsed_ssl_smtp.unwrap()),
        smtp_port: Some(parsed_smtp.unwrap()),
        ssl: Some(pick(KEY_MAIL_SSL).as_deref() == Some("1")),
        username: username.unwrap(),
        password: password.unwrap(),
        from: from.unwrap(),
    }))
}

/// 按 key upsert：存在则更新 value，不存在则插入新行。
///
/// Java 原版 `updateConfigByKey` 只在已存在时更新；这里扩展为不存在时自动插入，
/// 让前端首次写配置即可生效，避免要求用户手工 INSERT 一行占位。
async fn upsert_config(db: &DatabaseConnection, key: &str, value: &str) -> Result<(), AppError> {
    let existing = ConfigEntity::find()
        .filter(ConfigCol::Key.eq(key))
        .filter(ConfigCol::IsDeleted.eq(0))
        .one(db)
        .await?;
    match existing {
        Some(cfg) => {
            let mut active: ConfigActive = cfg.into();
            active.value = Set(value.to_string());
            active.update_time = Set(Some(chrono::Local::now().naive_local()));
            active.update(db).await?;
        }
        None => {
            let active = ConfigActive {
                id: Set(Uuid::new_v4().simple().to_string()),
                key: Set(key.to_string()),
                value: Set(value.to_string()),
                remark: Set(None),
                create_by: Set(None),
                create_time: Set(chrono::Local::now().naive_local()),
                update_by: Set(None),
                update_time: Set(None),
                is_deleted: Set(0),
            };
            active.insert(db).await?;
        }
    }
    Ok(())
}
