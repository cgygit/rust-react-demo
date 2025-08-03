use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, Header, EncodingKey};
use password_hash::{SaltString, PasswordHash, rand_core::OsRng};
use serde::{Serialize, Deserialize};
use std::env;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    password_hash
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(username: &str) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims {
        sub: username.to_owned(),
        exp: 2000000000, // UNIX 时间戳
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("Failed to encode JWT")
}
