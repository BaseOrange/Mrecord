//! 邮件配置相关 DTO / VO
//!
//! 对应 Java:
//! - `com.dcz.mrecord.dto.TestEmailDTO`
//! - `com.dcz.mrecord.dto.UpdateEmailConfigDTO`
//! - `com.dcz.mrecord.bo.EmailConfigBo`

use serde::{Deserialize, Serialize};

/// 测试邮件发送请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestEmailDto {
    /// SMTP 服务器地址
    pub host_name: String,
    /// SSL-SMTP 服务器端口
    pub ssl_smtp_port: Option<i32>,
    /// SMTP 服务器端口
    pub smtp_port: Option<i32>,
    /// 是否开启 SSL
    pub ssl: Option<bool>,
    /// 邮箱用户名
    pub user_name: String,
    /// 邮箱授权码
    pub password: String,
    /// 发送邮箱地址
    pub from: String,
    /// 测试收件邮箱
    pub test_to: String,
}

/// 修改邮件配置请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailConfigDto {
    /// SMTP 服务器地址
    pub host_name: String,
    /// SSL-SMTP 服务器端口
    pub ssl_smtp_port: Option<i32>,
    /// SMTP 服务器端口
    pub smtp_port: Option<i32>,
    /// 是否开启 SSL
    pub ssl: Option<bool>,
    /// 邮箱用户名
    pub user_name: String,
    /// 邮箱授权码
    pub password: String,
    /// 发送邮箱地址
    pub from: String,
}

/// 邮件配置响应/内部对象（对应 Java `EmailConfigBo`）
///
/// `getEmailConfig` 返回原始密码，`getMaskedEmailConfig` 返回密码掩码。
/// 字段顺序与 Java 端保持一致，便于前端复用同一组渲染逻辑。
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailConfigBo {
    /// SMTP 服务器主机名
    pub host_name: String,
    /// SSL-SMTP 服务器端口
    pub ssl_smtp_port: Option<i32>,
    /// SMTP 服务器端口
    pub smtp_port: Option<i32>,
    /// 是否使用 SSL
    pub ssl: Option<bool>,
    /// 邮箱用户名（注意：Java 端 BO 字段叫 `username`，但 DTO 与配置 key 都用 `userName`，
    /// 这里保留 Java BO 的命名以匹配前端预期的响应字段名）
    pub username: String,
    /// 邮箱授权码（脱敏后会被替换为 "******"）
    pub password: String,
    /// 发送邮箱地址
    pub from: String,
}
