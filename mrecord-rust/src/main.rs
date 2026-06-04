mod config;
mod constant;
mod db;
mod entity;
mod error;
mod handler;
mod middleware;
mod model;
mod router;

use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tracing_subscriber;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = db::connect().await;
    let state = AppState { db };

    let app = router::build(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Mrecord-rs server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
