use axum::{routing::get, Router};

use crate::{handler, AppState};

pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello from Axum + SeaORM!" }))
        .route("/records", get(handler::record::list).post(handler::record::create))
        .route("/records/{id}", get(handler::record::get).delete(handler::record::delete))
        .with_state(state)
}
