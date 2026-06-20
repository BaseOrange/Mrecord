//! 导出任务请求/响应 DTO
//!
//! 对应 Java DTO / VO:
//! - `com.dcz.mrecord.dto.QueryExportTaskDTO`
//! - `com.dcz.mrecord.vo.ExportTaskVO`

use serde::{Deserialize, Serialize};

use super::page_info::PageInfo;

/// 导出任务列表查询请求。
///
/// 对应 Java: `SysExportTaskController.list(PageInfoDTO)` 入参。
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryExportTaskDto {
    /// 分页参数
    #[serde(flatten)]
    pub page: PageInfo,
}

/// 导出任务列表响应。
///
/// 对应 Java: `SysExportTask` 列表展示字段。
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportTaskResponse {
    /// 任务 ID
    pub id: String,
    /// 账簿 ID
    pub book_id: String,
    /// 账簿名称
    pub book_name: Option<String>,
    /// 导出开始年月
    pub start_year_month: String,
    /// 导出结束年月
    pub end_year_month: String,
    /// 任务状态
    pub status: String,
    /// 生成文件名
    pub file_name: Option<String>,
    /// 失败原因
    pub fail_reason: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: Option<String>,
}

impl ExportTaskResponse {
    /// 从导出任务实体和账簿名构造列表响应。
    ///
    /// 对应 Java: `SysExportTask` + `bookName` 查询扩展字段。
    pub fn from_task(m: crate::entity::sys_export_task::Model, book_name: Option<String>) -> Self {
        Self {
            id: m.id,
            book_id: m.book_id,
            book_name,
            start_year_month: m.start_year_month,
            end_year_month: m.end_year_month,
            status: m.status,
            file_name: m.file_name,
            fail_reason: m.fail_reason,
            create_time: m.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: m
                .update_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
