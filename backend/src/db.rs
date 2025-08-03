use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub async fn init_db_pool() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}
