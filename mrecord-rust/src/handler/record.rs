use axum::{extract::State, http::StatusCode, Json};
use sea_orm::EntityTrait;

use crate::entity::record::{ActiveModel, Entity};
use crate::{
    error::AppError,
    model::record::{CreateRecord, RecordResponse},
    AppState,
};
use sea_orm::{ActiveModelTrait, Set};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<RecordResponse>>, AppError> {
    Entity::find()
        .all(&state.db)
        .await
        .map(|records| Json(records.into_iter().map(Into::into).collect()))
        .map_err(AppError::from)
}

pub async fn get(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<RecordResponse>, AppError> {
    Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(AppError::from)?
        .map(|r| Json(r.into()))
        .ok_or(AppError::NotFound)
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRecord>,
) -> Result<(StatusCode, Json<RecordResponse>), AppError> {
    let active = ActiveModel {
        title: Set(payload.title),
        amount: Set(payload.amount),
        ..Default::default()
    };

    let model = active.insert(&state.db).await.map_err(AppError::from)?;

    Ok((StatusCode::CREATED, Json(model.into())))
}

pub async fn delete(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<StatusCode, AppError> {
    let result = crate::entity::record::Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(AppError::from)?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
