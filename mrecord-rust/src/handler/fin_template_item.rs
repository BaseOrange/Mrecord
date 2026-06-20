//! 记账模板明细模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.FinTemplateItemController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.FinTemplateItemServiceImpl`

use axum::{Json, extract::State};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set,
    sea_query::Expr,
};
use uuid::Uuid;

use crate::{
    AppState,
    common::{res_code::ResCode, result::ApiResponse, user_context::AuthUser},
    entity::{
        fin_book::{Column as BookCol, Entity as BookEntity},
        fin_template_item::{self, ActiveModel as TemplateItemActive, Column as TemplateItemCol, Entity as TemplateItemEntity},
    },
    error::AppError,
    model::{
        finance::{FinTempItemDto, TemplateItemResponse},
    },
};

/// 构造参数错误业务异常。
fn param_err(msg: impl Into<String>) -> AppError {
    AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: msg.into(),
    }
}

/// 校验账簿存在且属于当前登录用户，并返回模板项列表。
///
/// 对应 Java: `FinTemplateItemServiceImpl.selectByFinBookIdExternal`
async fn check_book_and_get_template_items(
    state: &AppState,
    book_id: &str,
    user_id: &str,
) -> Result<Vec<fin_template_item::Model>, AppError> {
    // 通过 JOIN 查询校验所有权并获取模板项
    // 对应 SQL: SELECT fti.* FROM FIN_BOOK fb INNER JOIN FIN_TEMPLATE_ITEM fti ...
    let _book = BookEntity::find_by_id(book_id.to_string())
        .filter(BookCol::UserId.eq(user_id))
        .filter(BookCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?
        .ok_or(AppError::ResCode(ResCode::FinBookNotFound))?;

    // 如果账簿存在，查询模板项
    let items = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book_id))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .order_by_asc(Expr::cust("CAST(MR_SORT AS INTEGER)"))
        .all(&state.db)
        .await?;

    Ok(items)
}

/// 校验模板项类型合法。
fn validate_item_type(item_type: i32) -> Result<(), AppError> {
    // -1: 负债, 0: 仅记录, 1: 资产
    if !(-1..=1).contains(&item_type) {
        return Err(AppError::ResCode(ResCode::FinItemTempTypeError));
    }
    Ok(())
}

/// 创建账本模板项：`POST /tempItem/create`
///
/// 对应 Java: `FinTemplateItemController.create` 与 `FinTemplateItemServiceImpl.ceateFinTemplateItemList`
pub async fn create(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    // 校验账簿权限（通过查询操作隐式校验）
    let _ = check_book_and_get_template_items(&state, book_id, &user_id).await?;

    let item_list = params.item_list.ok_or_else(|| param_err("模板项列表不能为空"))?;
    if item_list.is_empty() {
        return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
    }

    let mut result = Vec::with_capacity(item_list.len());

    for item in item_list {
        if item.item_name.trim().is_empty() {
            return Err(AppError::ResCode(ResCode::FinItemTempNameRequired));
        }
        validate_item_type(item.item_type)?;

        let active = TemplateItemActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            book_id: Set(book_id.to_string()),
            item_name: Set(item.item_name),
            item_type: Set(item.item_type),
            icon: Set(item.icon),
            sort: Set(item.sort),
            create_by: Set(Some(user_id.clone())),
            ..Default::default()
        };

        let model = active.insert(&state.db).await?;
        result.push(model.into());
    }

    Ok(Json(ApiResponse::success(result)))
}

/// 更新账本模板项：`POST /tempItem/update`
///
/// 对应 Java: `FinTemplateItemController.update` 与 `FinTemplateItemServiceImpl.updateFinTemplateItemList`
pub async fn update(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    // 获取现有模板项并校验权限
    let existing_items = check_book_and_get_template_items(&state, book_id, &user_id).await?;
    let existing_map: std::collections::HashMap<_, _> = existing_items
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect();

    let item_list = params.item_list.ok_or_else(|| param_err("模板项列表不能为空"))?;
    if item_list.is_empty() {
        return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
    }

    let mut result = Vec::with_capacity(item_list.len());

    for item in item_list {
        match item.id {
            Some(ref id) if !id.trim().is_empty() => {
                let id_str: &str = id;
                // 更新现有项
                let existing = existing_map
                    .get(id_str)
                    .ok_or(AppError::ResCode(ResCode::FinItemTempUpdateError))?;

                // 禁止修改类型
                if existing.item_type != item.item_type {
                    return Err(AppError::ResCode(ResCode::FinItemTempUpdateError));
                }

                if item.item_name.trim().is_empty() {
                    return Err(AppError::ResCode(ResCode::FinItemTempNameRequired));
                }

                let mut active: TemplateItemActive = existing.clone().into();
                active.item_name = Set(item.item_name);
                active.icon = Set(item.icon);
                active.sort = Set(item.sort);
                active.update_by = Set(Some(user_id.clone()));
                active.update_time = Set(Some(chrono::Utc::now().naive_utc()));

                let model = active.update(&state.db).await?;
                result.push(model.into());
            }
            _ => {
                // 新增项
                if item.item_name.trim().is_empty() {
                    return Err(AppError::ResCode(ResCode::FinItemTempNameRequired));
                }
                validate_item_type(item.item_type)?;

                let active = TemplateItemActive {
                    id: Set(Uuid::new_v4().simple().to_string()),
                    book_id: Set(book_id.to_string()),
                    item_name: Set(item.item_name),
                    item_type: Set(item.item_type),
                    icon: Set(item.icon),
                    sort: Set(item.sort),
                    create_by: Set(Some(user_id.clone())),
                    ..Default::default()
                };

                let model = active.insert(&state.db).await?;
                result.push(model.into());
            }
        }
    }

    Ok(Json(ApiResponse::success(result)))
}

/// 复制账本模板项：`POST /tempItem/copy`
///
/// 对应 Java: `FinTemplateItemController.copy` 与 `FinTemplateItemServiceImpl.copyTemplateItem`
pub async fn copy(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let old_book_id = params.old_book_id.ok_or_else(|| param_err("原账簿ID不能为空"))?;
    let old_book_id = old_book_id.trim();
    if old_book_id.is_empty() {
        return Err(param_err("原账簿ID不能为空"));
    }

    let new_book_id = params.book_id.trim();
    if new_book_id.is_empty() {
        return Err(param_err("新账簿ID不能为空"));
    }

    // 校验旧账簿权限并获取模板项
    let source_items = check_book_and_get_template_items(&state, old_book_id, &user_id).await?;

    // 校验新账簿权限
    let _ = check_book_and_get_template_items(&state, new_book_id, &user_id).await?;

    if source_items.is_empty() {
        return Ok(Json(ApiResponse::success(vec![])));
    }

    let mut result = Vec::with_capacity(source_items.len());

    for item in source_items {
        let active = TemplateItemActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            book_id: Set(new_book_id.to_string()),
            item_name: Set(item.item_name),
            item_type: Set(item.item_type),
            icon: Set(item.icon),
            sort: Set(item.sort),
            create_by: Set(Some(user_id.clone())),
            ..Default::default()
        };

        let model = active.insert(&state.db).await?;
        result.push(model.into());
    }

    Ok(Json(ApiResponse::success(result)))
}

/// 查询账本模板项列表：`POST /tempItem/list`
///
/// 对应 Java: `FinTemplateItemController.list` 与 `FinTemplateItemServiceImpl.selectByFinBookIdExternal`
pub async fn list(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    let items = check_book_and_get_template_items(&state, book_id, &user_id).await?;

    if items.is_empty() {
        return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
    }

    let result = items.into_iter().map(TemplateItemResponse::from).collect();

    Ok(Json(ApiResponse::success(result)))
}
