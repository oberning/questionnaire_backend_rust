use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn build_connection_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(3)
        .connect(DB_URL)
        .await;
    pool
}
