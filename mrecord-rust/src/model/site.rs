//! 站点配置相关 DTO
//!
//! 对应 Java DTO: `com.dcz.mrecord.dto.UpdateSiteConfigDTO`

use serde::Deserialize;

/// 修改站点配置请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSiteConfigDto {
    /// 站点地址
    pub web_site: String,
    /// 管理员邮箱
    pub admin_mail: String,
    /// 是否开启注册功能
    pub register_enabled: Option<bool>,
}
