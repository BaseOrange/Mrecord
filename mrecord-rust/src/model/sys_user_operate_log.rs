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
    /// 逻辑删除标识（0-正常，1-已删除）
    ///
    /// 对应 Java `BaseEntity.isDeleted`：Java 直接返回实体，响应里天然带该字段。
    /// 前端 `BaseEntity` 类型与 `AdminLogsPage` 均未消费，补齐仅为响应契约对齐，
    /// 不影响任何页面渲染（多出的字段会被前端忽略）。
    pub is_deleted: i32,
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
            is_deleted: log.is_deleted,
        }
    }
}

#[cfg(test)]
mod tests {
    //! 响应 DTO 序列化测试，锁定 `isDeleted` 的对外契约。

    use super::*;
    use crate::entity::sys_user_operate_log::Model;
    use chrono::NaiveDateTime;
    use serde_json;

    fn sample_log(is_deleted: i32) -> Model {
        Model {
            id: "log1".into(),
            user_id: "u1".into(),
            operate_type: "/user/login".into(),
            content: "{}".into(),
            ip: "127.0.0.1".into(),
            create_by: Some("u1".into()),
            create_time: NaiveDateTime::parse_from_str("2026-04-01 12:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            update_by: None,
            update_time: None,
            is_deleted,
        }
    }

    #[test]
    fn response_serializes_is_deleted_as_camel_case() {
        let resp = OperateLogResponse::from_log(sample_log(0), Some("张三".into()), None);
        let json = serde_json::to_string(&resp).unwrap();
        assert!(
            json.contains("\"isDeleted\":0"),
            "响应应包含驼峰 isDeleted 字段: {json}"
        );
    }

    #[test]
    fn response_exposes_deleted_flag_when_logically_deleted() {
        let resp = OperateLogResponse::from_log(sample_log(1), None, None);
        assert_eq!(resp.is_deleted, 1);
    }
}
