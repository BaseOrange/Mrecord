mod db;
mod entity;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use entity::record::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[derive(Deserialize)]
struct CreateRecord {
    title: String,
    amount: f64,
}

#[derive(Serialize)]
struct RecordResponse {
    id: i32,
    title: String,
    amount: f64,
    created_at: String,
}

impl From<Model> for RecordResponse {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            title: m.title,
            amount: m.amount,
            created_at: m.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

// ---- Handlers ----

async fn list_records(
    State(state): State<AppState>,
) -> Result<Json<Vec<RecordResponse>>, StatusCode> {
    Entity::find()
        .all(&state.db)
        .await
        .map(|records| Json(records.into_iter().map(Into::into).collect()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_record(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<RecordResponse>, StatusCode> {
    Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|r| Json(r.into()))
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_record(
    State(state): State<AppState>,
    Json(payload): Json<CreateRecord>,
) -> Result<(StatusCode, Json<RecordResponse>), StatusCode> {
    let active = ActiveModel {
        title: Set(payload.title),
        amount: Set(payload.amount),
        ..Default::default()
    };

    let model = active
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(model.into())))
}

async fn delete_record(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn hello() -> &'static str {
    "Hello from Axum + SeaORM!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = db::connect().await;
    let state = AppState { db };

    let app = Router::new()
        .route("/", get(hello))
        .route("/records", get(list_records).post(create_record))
        .route("/records/{id}", get(get_record).delete(delete_record))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
