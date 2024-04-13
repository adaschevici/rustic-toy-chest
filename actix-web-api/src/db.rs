use actix_web::Result;
use dotenvy;
use sqlx::PgPool;
use std::env;

use crate::models::structs::ErrorMessage;

pub async fn connect() -> Result<PgPool, ErrorMessage> {
    if let Err(e) = dotenvy::dotenv() {
        return Err(ErrorMessage::new(&format!(
            "Error loading .env file: {}",
            e
        )));
    }

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| ErrorMessage::new("DATABASE_URL must be set in .env file"))?;

    let pool_result = PgPool::connect(&database_url)
        .await
        .map_err(|e| ErrorMessage::new(&format!("Error connecting to database: {}", e)))?;

    Ok(pool_result)
}
