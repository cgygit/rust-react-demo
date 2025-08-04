use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i32,
}

#[derive(Deserialize)]
pub struct CreateDevicePayload {
    pub name: String,
    pub description: Option<String>,
}
