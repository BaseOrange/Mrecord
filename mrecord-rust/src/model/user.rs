//! 用户相关请求/响应 DTO
//!
//! 对应 Java DTO:
//! - `com.dcz.mrecord.dto.UserDTO`
//! - `com.dcz.mrecord.dto.QueryUserDTO`
//! - `com.dcz.mrecord.dto.ChangePasswordDTO`
//! - `com.dcz.mrecord.dto.InitAdminDTO`

use serde::{Deserialize, Serialize};

use super::page_info::PageInfo;

// ==================== 请求 DTO ====================

/// 用户注册 / 登录请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    /// 用户 ID
    pub id: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 邮件提醒功能是否启用（0-关闭，1-开启）
    pub remind_enabled: Option<i32>,
    /// 月度提醒日期（取值 1-31，无对应日期取月末）
    pub remind_day: Option<i32>,
    /// 重置密码 token
    pub re_password_token: Option<String>,
    /// 激活账户 token
    pub activate_token: Option<String>,
}

/// 修改密码请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordDto {
    /// 旧密码
    pub old_password: String,
    /// 新密码
    pub new_password: String,
}

/// 初始化管理员账户请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAdminDto {
    /// 邮箱
    pub email: String,
    /// 密码
    pub password: String,
    /// 昵称
    pub nickname: String,
}

/// 查询用户请求参数（继承分页参数）
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserDto {
    /// 分页参数
    #[serde(flatten)]
    pub page: PageInfo,
    /// 昵称（模糊匹配）
    pub nickname: Option<String>,
    /// 邮箱（模糊匹配）
    pub email: Option<String>,
    /// 状态（0-正常，1-停用，2-注销待生效，3-已注销）
    pub status: Option<i32>,
    /// 是否管理员（0-正常用户，1-管理员）
    pub is_admin: Option<i32>,
}

// ==================== 响应 DTO ====================

/// 用户信息响应
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub nickname: String,
    /// 是否管理员（0-正常用户，1-管理员）
    pub admin: i32,
    /// 状态（0-正常，1-停用，2-注销待生效，3-已注销）
    pub status: i32,
    /// 邮件提醒功能是否启用（0-关闭，1-开启）
    pub remind_enabled: i32,
    /// 月度提醒日期（取值 1-31，无对应日期取月末）
    pub remind_day: Option<i32>,
    /// 账号注销申请时间
    pub cancel_time: Option<String>,
    /// 创建时间
    pub create_time: String,
}

impl From<crate::entity::sys_user::Model> for UserResponse {
    fn from(m: crate::entity::sys_user::Model) -> Self {
        Self {
            id: m.id,
            email: m.email,
            nickname: m.nickname,
            admin: m.admin,
            status: m.status,
            remind_enabled: m.remind_enabled,
            remind_day: m.remind_day,
            cancel_time: m.cancel_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            create_time: m.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
