use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use password_hash::{SaltString, PasswordHash, rand_core::OsRng};
use serde::{Serialize, Deserialize};
use std::env;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(username: &str) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: username.to_string(),
        exp: 2000000000,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

pub fn verify_jwt(token: &str) -> Option<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).ok()?;
    Some(decoded.claims.sub)
}
