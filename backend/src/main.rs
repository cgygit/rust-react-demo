mod db;
mod handlers;
mod models;
mod auth;

use axum::{routing::get, Router, routing::post};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use dotenv::dotenv;
use handlers::*;
use db::init_db_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let pool = init_db_pool().await;

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/api/hello", get(|| async { "Hello" }))
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .with_state(pool)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
