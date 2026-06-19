//! 站点配置相关 DTO / VO
//!
//! 对应 Java:
//! - `com.dcz.mrecord.dto.UpdateSiteConfigDTO`
//! - `com.dcz.mrecord.bo.SiteConfigBo`

use serde::{Deserialize, Serialize};

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

/// 站点配置响应（对应 Java `SiteConfigBo`）
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfigVo {
    /// 站点地址
    pub web_site: Option<String>,
    /// 管理员邮箱
    pub admin_mail: Option<String>,
    /// 是否开启注册功能
    pub register_enabled: bool,
}
