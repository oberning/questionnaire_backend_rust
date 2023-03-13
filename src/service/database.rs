use log::info;
use sea_orm::{prelude::*, ConnectOptions, Database};

const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/questionnaire";

pub async fn build_connection_pool() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(DB_URL.to_owned());
    opt.sqlx_logging(true);
    let db_connection = Database::connect(opt)
        .await
        .expect("Connection Pool could not be established!");
    info!("Created connection pool.");
    db_connection
}
