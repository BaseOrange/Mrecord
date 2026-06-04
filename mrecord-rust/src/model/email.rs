//! 邮件配置相关 DTO
//!
//! 对应 Java DTO:
//! - `com.dcz.mrecord.dto.TestEmailDTO`
//! - `com.dcz.mrecord.dto.UpdateEmailConfigDTO`

use serde::Deserialize;

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
