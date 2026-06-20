//! 用户操作审计日志请求/响应 DTO
//!
//! 对应 Java DTO / VO:
//! - `com.dcz.mrecord.dto.PageInfoDTO`
//! - `com.dcz.mrecord.entity.SysUserOperateLog`

use serde::{Deserialize, Serialize};

use super::page_info::PageInfo;

/// 操作日志列表查询请求。
///
/// 对应 Java: `SysUserOperateLogController.list(PageInfoDTO)` 入参。
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOperateLogDto {
    /// 分页参数
    #[serde(flatten)]
    pub page: PageInfo,
}

/// 操作日志列表响应。
///
/// 对应 Java: `SysUserOperateLog` 列表展示字段，额外补充创建人/更新人昵称。
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperateLogResponse {
    /// 日志 ID
    pub id: String,
    /// 操作用户 ID
    pub user_id: String,
    /// 操作类型，Java 端实际存请求 URI
    pub operate_type: String,
    /// 请求体内容
    pub content: String,
    /// 客户端 IP
    pub ip: String,
    /// 创建人
    pub create_by: Option<String>,
    /// 创建人昵称
    pub create_by_name: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新人
    pub update_by: Option<String>,
    /// 更新人昵称
    pub update_by_name: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

impl OperateLogResponse {
    /// 从操作日志实体和用户昵称构造列表响应。
    ///
    /// 对应 Java: `SysUserOperateLogServiceImpl.queryList` 中 LEFT JOIN SYS_USER 查询昵称。
    pub fn from_log(
        log: crate::entity::sys_user_operate_log::Model,
        create_by_name: Option<String>,
        update_by_name: Option<String>,
    ) -> Self {
        Self {
            id: log.id,
            user_id: log.user_id,
            operate_type: log.operate_type,
            content: log.content,
            ip: log.ip,
            create_by: log.create_by,
            create_by_name,
            create_time: log.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_by: log.update_by,
            update_by_name,
            update_time: log
                .update_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
