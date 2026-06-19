//! 工具模块
//!
//! 对应 Java 包: `com.dcz.mrecord.util`
//!
//! 当前包含：
//! - `jwt`: 登录 token 生成与解析（对应 `JwtUtil`）
//! - `token`: 激活/重置密码等带有用途的短期 JWT（替代 Java 中的 AES 加密 token）

pub mod jwt;
pub mod token;
