//! 应用安全配置
//!
//! 对应 Java: `com.dcz.mrecord.config.MrConf`
//!
//! 启动时从 `SYS_CONFIG` 表加载安全配置项（登录 JWT 密钥、账户激活 / 重置密码
//! 令牌密钥、JWT 过期时长），供 [`crate::AppState`] 使用。
//!
//! 设计要点（与 Java `MrConf` 的 `@PostConstruct` 行为保持一致）：
//! - 三个密钥缺失或为空时，自动生成 UUID（去掉 `-`）并回写数据库。这样冷启动
//!   一次后密钥即可持久化，重启后保持稳定，已签发的 token 才能跨重启被解析。
//! - `mr.jwtExpire` 缺失或为空时使用默认值 7 天（`604800000` 毫秒）。
//! - 仅启动时读取一次；管理后台修改这些配置后需重启服务生效（与 Java 相同）。
//! - `mr.*` 密钥不经过 [`crate::service::sys_config::SysConfigService`] 的内存缓存，
//!   该缓存只覆盖邮件 / 站点 / 注册开关配置项。

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    error::AppError,
    service::sys_config::{load_single, upsert_config},
};

// ==================== 配置项 key 常量 ====================
//
// 与 Java `MrConf` 中的字符串字面量保持完全一致。

const KEY_JWT_SECRET: &str = "mr.jwtSecret";
const KEY_ACTIVATE_TOKEN_SECRET: &str = "mr.activateTokenSecret";
const KEY_RESET_PWD_TOKEN_SECRET: &str = "mr.resetPwdTokenSecret";
const KEY_JWT_EXPIRE: &str = "mr.jwtExpire";

/// JWT 默认过期时间：7 天（单位：毫秒），与 Java `MrConf` 缺省值一致
const DEFAULT_JWT_EXPIRE_MILLIS: i64 = 7 * 24 * 60 * 60 * 1000;

/// 安全配置项
///
/// 对应 Java `MrConf` 的四个字段，由 [`load_security_config`] 在启动时一次性加载。
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// 登录 JWT 密钥（对应 Java `MrConf.jwtSecret`）
    pub jwt_secret: String,
    /// 账户激活令牌密钥（对应 Java `MrConf.activateTokenSecret`）
    pub activate_token_secret: String,
    /// 重置密码令牌密钥（对应 Java `MrConf.resetPwdTokenSecret`）
    pub reset_pwd_token_secret: String,
    /// JWT 过期时长（单位：毫秒，对应 Java `MrConf.jwtExpire`）
    pub jwt_expire_millis: i64,
}

impl SecurityConfig {
    /// JWT 过期时长（单位：秒），供 [`crate::util::jwt::create_token`] 使用
    pub fn jwt_expire_secs(&self) -> i64 {
        self.jwt_expire_millis / 1000
    }
}

/// 从 `SYS_CONFIG` 加载安全配置
///
/// 对应 Java: `MrConf.init`（`@PostConstruct`）。
///
/// 三个密钥的加载策略与 Java `MrConf.loadOrGenerate` 一致：值为空则生成去掉
/// 横线的 UUID 并回写数据库；`jwtExpire` 为空时取默认值。
pub async fn load_security_config(db: &DatabaseConnection) -> Result<SecurityConfig, AppError> {
    let jwt_secret = load_or_generate_secret(db, KEY_JWT_SECRET).await?;
    let activate_token_secret = load_or_generate_secret(db, KEY_ACTIVATE_TOKEN_SECRET).await?;
    let reset_pwd_token_secret = load_or_generate_secret(db, KEY_RESET_PWD_TOKEN_SECRET).await?;
    let jwt_expire_millis = load_jwt_expire(db).await?;

    tracing::info!("已从 SYS_CONFIG 表加载安全配置项");
    Ok(SecurityConfig {
        jwt_secret,
        activate_token_secret,
        reset_pwd_token_secret,
        jwt_expire_millis,
    })
}

/// 加载密钥：缺失或为空时生成 UUID（去掉 `-`）并回写数据库
///
/// 对应 Java: `MrConf.loadOrGenerate`
async fn load_or_generate_secret(db: &DatabaseConnection, key: &str) -> Result<String, AppError> {
    if let Some(value) = load_single(db, key).await?
        && !value.is_empty()
    {
        return Ok(value);
    }
    let value = Uuid::new_v4().simple().to_string();
    upsert_config(db, key, &value).await?;
    tracing::info!("配置项 {} 未设置，已自动生成并保存到数据库", key);
    Ok(value)
}

/// 加载 JWT 过期时长（毫秒）
///
/// 对应 Java: `MrConf.init` 中对 `mr.jwtExpire` 的解析：为空时取默认值 7 天。
/// 值存在但无法解析为整数时直接报错终止启动，与 Java `Long.parseLong` 抛出
/// `NumberFormatException` 导致启动失败的行为对齐。
async fn load_jwt_expire(db: &DatabaseConnection) -> Result<i64, AppError> {
    match load_single(db, KEY_JWT_EXPIRE).await? {
        Some(value) if !value.is_empty() => value.parse::<i64>().map_err(|_| {
            AppError::Internal(anyhow::anyhow!(
                "配置项 {} 的值「{}」不是合法的整数",
                KEY_JWT_EXPIRE,
                value
            ))
        }),
        _ => Ok(DEFAULT_JWT_EXPIRE_MILLIS),
    }
}
