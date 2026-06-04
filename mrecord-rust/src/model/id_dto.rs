//! 通用 ID 请求体
//!
//! 对应 Java DTO: `com.dcz.mrecord.dto.IdDto`

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdDto {
    /// 主键 ID
    pub id: String,
}
