use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;

pub async fn create_pool() -> Pool<Postgres> {
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}