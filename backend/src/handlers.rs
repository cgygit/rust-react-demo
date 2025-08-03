use axum::{extract::State, Json};
use sqlx::{PgPool, Row};
use crate::models::*;
use crate::auth::*;

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> Json<&'static str> {
    let hashed = hash_password(&payload.password);

    let _ = sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
    )
    .bind(&payload.username)
    .bind(&hashed)
    .execute(&pool)
    .await
    .unwrap();

    Json("User registered")
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> Json<TokenResponse> {
    let row = sqlx::query(
        "SELECT password_hash FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_one(&pool)
    .await
    .unwrap();

    let password_hash: String = row.get("password_hash");

    if verify_password(&payload.password, &password_hash) {
        let token = create_jwt(&payload.username);
        Json(TokenResponse { token })
    } else {
        panic!("Unauthorized");
    }
}
