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
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, Set, TransactionTrait,
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

        // 对应 Java @Transactional(rollbackFor = Exception.class)：7 条配置项
        // 要么全部写入成功，要么整体回滚，避免出现半更新的配置。
        let txn = db.begin().await?;
        upsert_config(&txn, KEY_MAIL_HOST_NAME, &dto.host_name).await?;
        upsert_config(
            &txn,
            KEY_MAIL_SSL_SMTP_PORT,
            &dto.ssl_smtp_port.map(|v| v.to_string()).unwrap_or_default(),
        )
        .await?;
        upsert_config(
            &txn,
            KEY_MAIL_SMTP_PORT,
            &dto.smtp_port.map(|v| v.to_string()).unwrap_or_default(),
        )
        .await?;
        upsert_config(
            &txn,
            KEY_MAIL_SSL,
            if dto.ssl.unwrap_or(false) { "1" } else { "0" },
        )
        .await?;
        upsert_config(&txn, KEY_MAIL_USER_NAME, &dto.user_name).await?;
        upsert_config(&txn, KEY_MAIL_PASSWORD, &password).await?;
        upsert_config(&txn, KEY_MAIL_FROM, &dto.from).await?;
        // 提交前任何一步失败（`?` 抛错）都会令事务在 drop 时回滚
        txn.commit().await?;

        // 失效缓存：放在事务提交成功之后。回滚时 DB 未变，缓存依然有效；
        // 只有真正落库的成功路径才会清缓存，避免缓存与 DB 不一致。
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
        // 对应 Java @Transactional(rollbackFor = Exception.class)：3 条配置项
        // 要么全部写入成功，要么整体回滚，避免出现半更新的配置。
        let txn = db.begin().await?;
        upsert_config(&txn, KEY_WEB_SITE, &dto.web_site).await?;
        upsert_config(&txn, KEY_ADMIN_MAIL, &dto.admin_mail).await?;
        upsert_config(
            &txn,
            KEY_REGISTER_ENABLED,
            if dto.register_enabled.unwrap_or(false) {
                "1"
            } else {
                "0"
            },
        )
        .await?;
        txn.commit().await?;

        // 事务提交成功后再失效缓存，失败回滚时缓存保持有效
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

// ==================== 辅助函数 ====================
//
// `load_single` / `upsert_config` 对应 Java `SysConfigServiceImpl` 中的
// `loadConfigValue` / `updateConfigByKey`，除服务自身使用外，也供启动时的
// `config::load_security_config` 复用，避免重复实现读写逻辑。

/// 按 key 加载单条配置的 value
pub(crate) async fn load_single(
    db: &DatabaseConnection,
    key: &str,
) -> Result<Option<String>, AppError> {
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
///
/// 【3.8/3.9 核实结论——此处与 Java 语义等价，勿改】Java 端查询用的是
/// `QueryWrapper.likeLeft(SysConfig::getKey, "mail")`。MyBatis-Flex 的命名与直觉相反：
/// `likeLeft(value)` 生成的是 `LIKE 'value%'`（前缀匹配），而 `likeRight(value)`
/// 才是 `LIKE '%value'`（后缀匹配）——核实自 mybatis-flex-core 1.11.6 源码
/// `QueryColumn.likeLeft_`（`value + "%"`）。因此 Java 实际 SQL 是
/// `MR_KEY LIKE 'mail%'`，**能正确命中全部 `mail.*` 行**，并非 bug。
/// Rust 这里用 `starts_with("mail.")`（等价 `LIKE 'mail.%'`）：对现有配置键集合
/// （只有 `mail.xxx`，无 `mailFoo` 这类无点前缀键）两者结果完全一致；
/// Rust 的写法更精确（强制含点分隔符）。特此注释，避免后人按命名直觉误改。
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
///
/// 参数为 `ConnectionTrait` 泛型，既可以接收 `&DatabaseConnection` 直接执行，
/// 也可以接收 `&DatabaseTransaction`（`db.begin()` 的返回值）放进事务里执行。
pub(crate) async fn upsert_config<C>(db: &C, key: &str, value: &str) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
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
                create_time: Set(Some(chrono::Local::now().naive_local())),
                update_by: Set(None),
                update_time: Set(None),
                is_deleted: Set(0),
            };
            active.insert(db).await?;
        }
    }
    Ok(())
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use sea_orm::{ConnectionTrait, Database};

    use super::*;

    /// 建一张内存 SQLite 库（共享缓存，保证连接池内所有连接看到同一份数据）
    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:?cache=shared")
            .await
            .expect("连接内存库失败");
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS SYS_CONFIG (
                MR_ID           TEXT PRIMARY KEY,
                MR_CONFIG_KEY   TEXT,
                MR_CONFIG_VALUE TEXT,
                MR_REMARK       TEXT,
                MR_CREATE_BY    TEXT,
                MR_CREATE_TIME  TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY    TEXT,
                MR_UPDATE_TIME  TEXT,
                MR_IS_DELETED   INTEGER DEFAULT 0
            )",
        )
        .await
        .expect("建表失败");
        db
    }

    /// 读取某个 key 当前的 value（测试断言用）
    async fn read_value(db: &DatabaseConnection, key: &str) -> Option<String> {
        load_single(db, key).await.expect("读取配置失败")
    }

    /// 验收：update_site_config 中途 upsert 失败时，DB 无半更新残留。
    ///
    /// 手法：在 `adminMail`（第 2 条）上挂触发器模拟 DB 写失败。若未包事务，
    /// 第 1 条 `webSite` 会落库；包了事务则整体回滚，两条都不应残留新值。
    #[tokio::test]
    async fn update_site_config_rolls_back_on_failure() {
        let db = setup_db().await;
        let svc = SysConfigService::new();

        // 预置旧行，让 upsert 走 UPDATE 路径
        upsert_config(&db, KEY_WEB_SITE, "https://old.example.com")
            .await
            .unwrap();
        upsert_config(&db, KEY_ADMIN_MAIL, "old@example.com")
            .await
            .unwrap();

        // 模拟第 2 条 upsert 失败：对 adminMail 的 UPDATE / INSERT 直接 ABORT
        db.execute_unprepared(
            "CREATE TRIGGER sim_fail_admin_mail_update BEFORE UPDATE ON SYS_CONFIG
             FOR EACH ROW
             BEGIN
                 SELECT RAISE(ABORT, 'simulated failure')
                 WHERE NEW.MR_CONFIG_KEY = 'adminMail';
             END",
        )
        .await
        .unwrap();
        db.execute_unprepared(
            "CREATE TRIGGER sim_fail_admin_mail_insert BEFORE INSERT ON SYS_CONFIG
             FOR EACH ROW
             BEGIN
                 SELECT RAISE(ABORT, 'simulated failure')
                 WHERE NEW.MR_CONFIG_KEY = 'adminMail';
             END",
        )
        .await
        .unwrap();

        let result = svc
            .update_site_config(
                &db,
                UpdateSiteConfigDto {
                    web_site: "https://new.example.com".to_string(),
                    admin_mail: "new@example.com".to_string(),
                    register_enabled: Some(true),
                },
            )
            .await;

        // 接口应当返回错误
        assert!(result.is_err(), "中途失败时应返回错误");

        // 关键断言：DB 里不能有半更新的残留，两个 key 都还是旧值
        assert_eq!(
            read_value(&db, KEY_WEB_SITE).await.as_deref(),
            Some("https://old.example.com"),
            "webSite 不应被半更新"
        );
        assert_eq!(
            read_value(&db, KEY_ADMIN_MAIL).await.as_deref(),
            Some("old@example.com"),
            "adminMail 不应被半更新"
        );
        assert_eq!(
            read_value(&db, KEY_REGISTER_ENABLED).await,
            None,
            "registerEnabled 不应被写入"
        );
    }

    /// 验收：update_email_config 中途 upsert 失败时，DB 无半更新残留。
    ///
    /// 在 `mail.password`（第 6 条）上挂触发器，前 5 条若未包事务会落库。
    #[tokio::test]
    async fn update_email_config_rolls_back_on_failure() {
        let db = setup_db().await;
        let svc = SysConfigService::new();

        let result = svc
            .update_email_config(
                &db,
                UpdateEmailConfigDto {
                    host_name: "smtp.new.com".to_string(),
                    ssl_smtp_port: Some(465),
                    smtp_port: Some(25),
                    ssl: Some(true),
                    user_name: "new@new.com".to_string(),
                    password: "secret".to_string(),
                    from: "new@new.com".to_string(),
                },
            )
            .await;
        assert!(result.is_ok());

        // 确认首次写入成功（触发器还没挂）
        assert_eq!(
            read_value(&db, KEY_MAIL_HOST_NAME).await.as_deref(),
            Some("smtp.new.com")
        );

        // 挂触发器模拟第 6 条 mail.password 失败
        db.execute_unprepared(
            "CREATE TRIGGER sim_fail_mail_pwd_update BEFORE UPDATE ON SYS_CONFIG
             FOR EACH ROW
             BEGIN
                 SELECT RAISE(ABORT, 'simulated failure')
                 WHERE NEW.MR_CONFIG_KEY = 'mail.password';
             END",
        )
        .await
        .unwrap();

        let result = svc
            .update_email_config(
                &db,
                UpdateEmailConfigDto {
                    host_name: "smtp.other.com".to_string(),
                    ssl_smtp_port: Some(587),
                    smtp_port: Some(2525),
                    ssl: Some(false),
                    user_name: "other@other.com".to_string(),
                    password: "changed".to_string(),
                    from: "other@other.com".to_string(),
                },
            )
            .await;

        assert!(result.is_err(), "中途失败时应返回错误");

        // 前 5 条都不应残留新值（仍是第一次写入的值）
        assert_eq!(
            read_value(&db, KEY_MAIL_HOST_NAME).await.as_deref(),
            Some("smtp.new.com"),
            "mail.hostName 不应被半更新"
        );
        assert_eq!(
            read_value(&db, KEY_MAIL_USER_NAME).await.as_deref(),
            Some("new@new.com"),
            "mail.userName 不应被半更新"
        );
        assert_eq!(
            read_value(&db, KEY_MAIL_FROM).await.as_deref(),
            Some("new@new.com"),
            "mail.from 不应被半更新"
        );
        assert_eq!(
            read_value(&db, KEY_MAIL_PASSWORD).await.as_deref(),
            Some("secret"),
            "mail.password 保持原值"
        );
    }

    /// 正向用例：全部成功时数据落库且缓存被清空
    #[tokio::test]
    async fn update_site_config_commits_and_invalidates_cache() {
        let db = setup_db().await;
        let svc = SysConfigService::new();

        svc.update_site_config(
            &db,
            UpdateSiteConfigDto {
                web_site: "https://ok.example.com".to_string(),
                admin_mail: "ok@example.com".to_string(),
                register_enabled: Some(true),
            },
        )
        .await
        .unwrap();

        // 落库
        assert_eq!(
            read_value(&db, KEY_WEB_SITE).await.as_deref(),
            Some("https://ok.example.com")
        );
        assert_eq!(
            read_value(&db, KEY_ADMIN_MAIL).await.as_deref(),
            Some("ok@example.com")
        );
        assert_eq!(
            read_value(&db, KEY_REGISTER_ENABLED).await.as_deref(),
            Some("1")
        );

        // 缓存被清空，重新读会从 DB 加载
        assert!(svc.get_web_site(&db).await.unwrap().is_some());
    }

    // ==================== 3.9 邮件配置键范围核实 ====================

    /// 写入一行配置（不依赖 upsert 之外的服务方法）。
    async fn seed_config(db: &DatabaseConnection, key: &str, value: &str) {
        upsert_config(db, key, value).await.unwrap();
    }

    /// `load_email_config` 只取 `mail.*` 行：同表存在 `adminMail`（后缀含 mail）、
    /// `mr.jwtSecret` 等干扰键时也不应混入，且必填项齐全时能正确组装。
    ///
    /// 对照 Java `likeLeft(key, "mail")` → `MR_KEY LIKE 'mail%'`：
    /// 语义等价（两者都只命中 `mail.*`），此测试锁定该范围不被误改。
    #[tokio::test]
    async fn load_email_config_picks_only_mail_prefixed_keys() {
        let db = setup_db().await;

        // 干扰键：后缀匹配模式（错误的 LIKE '%mail'）会把 adminMail 混进来
        seed_config(&db, KEY_ADMIN_MAIL, "admin@example.com").await;
        seed_config(&db, "mr.jwtSecret", "secret").await;
        // 全套 mail.* 配置
        seed_config(&db, KEY_MAIL_HOST_NAME, "smtp.example.com").await;
        seed_config(&db, KEY_MAIL_SSL_SMTP_PORT, "465").await;
        seed_config(&db, KEY_MAIL_SMTP_PORT, "25").await;
        seed_config(&db, KEY_MAIL_SSL, "1").await;
        seed_config(&db, KEY_MAIL_USER_NAME, "user@example.com").await;
        seed_config(&db, KEY_MAIL_PASSWORD, "pwd").await;
        seed_config(&db, KEY_MAIL_FROM, "from@example.com").await;

        let cfg = load_email_config(&db).await.expect("加载邮件配置失败");
        let cfg = cfg.expect("必填项齐全时应返回 Some");
        assert_eq!(cfg.host_name, "smtp.example.com");
        assert_eq!(cfg.ssl_smtp_port, Some(465));
        assert_eq!(cfg.smtp_port, Some(25));
        assert_eq!(cfg.ssl, Some(true));
        assert_eq!(cfg.username, "user@example.com");
        assert_eq!(cfg.password, "pwd");
        assert_eq!(cfg.from, "from@example.com");
    }

    /// `adminMail` 的值不应被当成邮件参数：即使 mail.* 缺失，也不能用 adminMail 顶替。
    #[tokio::test]
    async fn load_email_config_returns_none_when_mail_keys_missing() {
        let db = setup_db().await;
        // 只有干扰键，没有任何 mail.* 行
        seed_config(&db, KEY_ADMIN_MAIL, "admin@example.com").await;

        assert!(
            load_email_config(&db)
                .await
                .expect("查询不应失败")
                .is_none(),
            "无 mail.* 配置时应返回 None，且不能把 adminMail 当邮件参数"
        );
    }

    /// 端口为 None 时写空串而非 Java 的字面量 "null"（3.9：保持 Rust 更合理的实现）。
    #[tokio::test]
    async fn update_email_config_writes_empty_string_for_null_ports() {
        let db = setup_db().await;
        let svc = SysConfigService::new();

        svc.update_email_config(
            &db,
            UpdateEmailConfigDto {
                host_name: "smtp.example.com".to_string(),
                ssl_smtp_port: None,
                smtp_port: None,
                ssl: Some(false),
                user_name: "user@example.com".to_string(),
                password: "pwd".to_string(),
                from: "from@example.com".to_string(),
            },
        )
        .await
        .unwrap();

        // 空串而非 "null"——后续 load_email_config 会把空串当作「未配置」优雅返回 None
        assert_eq!(
            read_value(&db, KEY_MAIL_SSL_SMTP_PORT).await.as_deref(),
            Some("")
        );
        assert_eq!(
            read_value(&db, KEY_MAIL_SMTP_PORT).await.as_deref(),
            Some("")
        );
        assert!(
            load_email_config(&db).await.unwrap().is_none(),
            "端口缺失（空串）时加载应返回 None 而非解析报错"
        );
    }
}
