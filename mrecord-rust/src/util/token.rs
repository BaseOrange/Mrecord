//! 短期用途令牌工具（账户激活 / 找回密码）
//!
//! 对应 Java 中：`SysUserServiceImpl#getActivateAccountUrl` /
//! `getResetPasswordUrl` 用 `SecureUtil.aes` 加密 `userId_expireTime_random`，
//! 并在 `checkActivateToken` / `checkRePasswordToken` 中解密校验。
//!
//! Rust 版改用带 `purpose` 声明的 JWT（HS256），含义对等：
//! - 内置过期时间，无需自行比较时间戳
//! - 通过 `purpose` 字段防止令牌串用（重置密码 token 不能拿来激活）

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// 令牌用途
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenPurpose {
    /// 账户激活（24 小时）
    Activate,
    /// 重置密码（15 分钟）
    ResetPassword,
}

impl TokenPurpose {
    /// 不同用途对应的过期秒数
    fn ttl_secs(self) -> i64 {
        match self {
            // 24 小时
            TokenPurpose::Activate => 24 * 60 * 60,
            // 15 分钟
            TokenPurpose::ResetPassword => 15 * 60,
        }
    }
}

/// 用途令牌载荷
#[derive(Debug, Serialize, Deserialize)]
pub struct PurposeClaims {
    /// 主题：userId
    pub sub: String,
    /// 用途
    pub purpose: TokenPurpose,
    /// 过期时间（unix 秒）
    pub exp: i64,
}

/// 生成用途令牌
pub fn create(
    user_id: &str,
    purpose: TokenPurpose,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = chrono::Utc::now().timestamp() + purpose.ttl_secs();
    let claims = PurposeClaims {
        sub: user_id.to_string(),
        purpose,
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// 解析并校验用途令牌
///
/// 校验通过返回 `userId`；以下情况返回 `None`：
/// - 解析失败 / 签名错误 / 已过期
/// - 用途不匹配（防止令牌串用）
pub fn parse(token: &str, expected: TokenPurpose, secret: &str) -> Option<String> {
    let data = decode::<PurposeClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;
    if data.claims.purpose != expected {
        return None;
    }
    Some(data.claims.sub)
}
