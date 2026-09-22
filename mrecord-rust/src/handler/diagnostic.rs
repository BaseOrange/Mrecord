//! 诊断信息模块 HTTP 处理函数
//!
//! 前端「我的」页面顶部标题点击 5 次后打开诊断面板，调用本接口获取后端环境
//! 信息，供用户在 GitHub 提 issue 时一键复制（对应前端 `DiagnosticSheet.vue`）。
//!
//! 【设计要点】
//! - 需要 `AuthUser` 登录：`data_counts` 按当前用户过滤。未登录场景（如登录页
//!   白屏）下，前端纯环境信息已足够排障，无需调用本接口。
//! - 只返回环境元数据与**计数**：绝不包含金额、账号、邮箱、账簿名、条目名等
//!   业务或敏感数据（详见 `model::diagnostic` 的隐私红线说明）。
//! - 本接口被操作日志中间件跳过（`middleware::log::should_skip_log`），查看
//!   诊断信息本身不会产生审计日志，避免「查看 → 产生日志」的循环。

use axum::{Json, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

use crate::{
    AppState,
    common::{result::ApiResponse, user_context::AuthUser},
    entity::{
        fin_book::{Column as BookCol, Entity as BookEntity},
        fin_month_item_record::{Column as MonthItemCol, Entity as MonthItemEntity},
        fin_month_record::{Column as MonthRecordCol, Entity as MonthRecordEntity},
        fin_template_item::{Column as TemplateItemCol, Entity as TemplateItemEntity},
    },
    model::diagnostic::{DataCounts, DiagnosticResponse},
};

/// 查询诊断信息
///
/// - 请求：无（通过 `AuthUser` 提取当前登录用户）
/// - 响应：[`DiagnosticResponse`]
pub async fn query_diagnostic(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DiagnosticResponse>>, crate::error::AppError> {
    // 先取当前用户名下未删除的账簿：`FIN_TEMPLATE_ITEM` 与 `FIN_MONTH_ITEM_RECORD`
    // 只带 `MR_BOOK_ID`（通过账簿间接关联用户），需要先拿到账簿 id 集合才能按用户
    // 过滤——与 `fin_book::get_my_data_statistics` 的做法同构。
    let book_ids: Vec<String> = BookEntity::find()
        .filter(BookCol::UserId.eq(user_id.clone()))
        .filter(BookCol::IsDeleted.eq(0))
        .all(&state.db)
        .await?
        .into_iter()
        .map(|book| book.id)
        .collect();

    let book_count = book_ids.len() as u64;

    // 月度汇总表带 `MR_USER_ID`，可直接按用户过滤
    let month_record_count = MonthRecordEntity::find()
        .filter(MonthRecordCol::UserId.eq(user_id))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .count(&state.db)
        .await?;

    // 没有账簿时关联计数必为 0，直接短路，避免一次空 IN 查询
    let (template_item_count, month_item_count) = if book_ids.is_empty() {
        (0, 0)
    } else {
        let template_item_count = TemplateItemEntity::find()
            .filter(TemplateItemCol::BookId.is_in(book_ids.clone()))
            .filter(TemplateItemCol::IsDeleted.eq(0))
            .count(&state.db)
            .await?;

        let month_item_count = MonthItemEntity::find()
            .filter(MonthItemCol::BookId.is_in(book_ids))
            .filter(MonthItemCol::IsDeleted.eq(0))
            .count(&state.db)
            .await?;

        (template_item_count, month_item_count)
    };

    // 数据库文件大小：与 `db::connect()` 的连接串 `sqlite://data.db` 对齐，
    // 相对当前工作目录解析；读取失败（如权限/路径异常）时返回 0 而非报错——
    // 诊断信息应尽可能可用，文件大小不是关键字段。
    let db_size_bytes = std::env::current_dir()
        .ok()
        .and_then(|dir| std::fs::metadata(dir.join("data.db")).ok())
        .map(|meta| meta.len())
        .unwrap_or(0);

    // 部署模式：飞牛 fnOS 网关模式带有前缀（`MRECORD_GATEWAY_PREFIX`），
    // 独立部署 / Docker 部署为 standalone。两种形态的行为差异较大，排障时需要区分。
    let deploy_mode = crate::env_nonempty("MRECORD_GATEWAY_PREFIX")
        .map(|prefix| format!("fnos-gateway ({prefix})"))
        .unwrap_or_else(|| "standalone".to_string());

    Ok(Json(ApiResponse::success(DiagnosticResponse {
        // 后端类型标识：本二进制是 Rust 版（fnOS 版），Java 版（Docker 版）返回 "java"
        backend: "rust".to_string(),
        backend_framework: "axum 0.8".to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: env!("RUSTC_VERSION").to_string(),
        build_time: env!("BUILD_TIME").to_string(),
        uptime_secs: state.started_at.elapsed().as_secs(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        deploy_mode,
        db_type: "sqlite".to_string(),
        db_size_bytes,
        data_counts: DataCounts {
            book_count,
            template_item_count,
            month_record_count,
            month_item_count,
        },
    })))
}
