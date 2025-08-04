use axum::{
    extract::{State, TypedHeader},
    Json,
};
use headers::{Authorization, authorization::Bearer};
use sqlx::{PgPool, Row};
use crate::models::*;
use crate::auth::*;

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> Json<&'static str> {
    let hashed = hash_password(&payload.password);
    sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
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
    let row = sqlx::query("SELECT password_hash FROM users WHERE username = $1")
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

pub async fn create_device(
    State(pool): State<PgPool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<CreateDevicePayload>,
) -> Json<&'static str> {
    let token = bearer.token();
    let username = verify_jwt(token).unwrap();

    let row = sqlx::query("SELECT id FROM users WHERE username = $1")
        .bind(&username)
        .fetch_one(&pool)
        .await
        .unwrap();

    let user_id: i32 = row.get("id");

    sqlx::query("INSERT INTO devices (name, description, owner_id) VALUES ($1, $2, $3)")
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    Json("Device created")
}

pub async fn get_devices(
    State(pool): State<PgPool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Json<Vec<Device>> {
    let token = bearer.token();
    let username = verify_jwt(token).unwrap();

    let row = sqlx::query("SELECT id FROM users WHERE username = $1")
        .bind(&username)
        .fetch_one(&pool)
        .await
        .unwrap();

    let user_id: i32 = row.get("id");

    let rows = sqlx::query("SELECT id, name, description, owner_id FROM devices WHERE owner_id = $1")
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let devices = rows.into_iter().map(|row| Device {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        owner_id: row.get("owner_id"),
    }).collect();

    Json(devices)
}
