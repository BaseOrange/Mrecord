//! JWT 登录令牌工具
//!
//! 对应 Java: `com.dcz.mrecord.util.JwtUtil`
//!
//! 仅存放用户主键（`sub` 字段），过期时间默认 7 天。

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// 默认登录令牌过期时间：7 天（单位：秒）
const LOGIN_TOKEN_EXPIRE_SECS: i64 = 7 * 24 * 60 * 60;

/// JWT 载荷
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 主题，存放 userId
    pub sub: String,
    /// 过期时间（unix 秒）
    pub exp: i64,
}

/// 生成登录令牌
///
/// 对应 Java: `JwtUtil.createToken`
pub fn create_token(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // 当前时间 + 默认过期时长
    let exp = chrono::Utc::now().timestamp() + LOGIN_TOKEN_EXPIRE_SECS;
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// 解析登录令牌，返回 userId
///
/// 对应 Java: `JwtUtil.getUserId`
///
/// 失败（非法或过期）时返回 None，与 Java 行为保持一致。
pub fn parse_user_id(token: &str, secret: &str) -> Option<String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
        .ok()
        .map(|data| data.claims.sub)
}
