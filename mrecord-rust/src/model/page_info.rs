//! 分页请求参数
//!
//! 对应 Java DTO: `com.dcz.mrecord.dto.PageInfoDTO`

use serde::Deserialize;

fn default_page_num() -> i32 {
    1
}
fn default_page_size() -> i32 {
    10
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    /// 页码，默认 1
    #[serde(default = "default_page_num")]
    pub page_num: i32,
    /// 每页数量，默认 10
    #[serde(default = "default_page_size")]
    pub page_size: i32,
}
