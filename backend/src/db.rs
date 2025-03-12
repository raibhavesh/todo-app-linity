use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn connect_to_db(database_url: &str) -> Result<Pool<Postgres>> {
    // Configure connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}