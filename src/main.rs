mod model;
mod service;
mod migrator;
mod auth;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use actix_web_httpauth::middleware::HttpAuthentication;
use migrator::Migrator;
use sea_orm_migration::prelude::*;
use service::{routes, database};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    let auth = HttpAuthentication::bearer(auth::validator);
    let db_pool = database::build_connection_pool().await;
    let schema_manager = SchemaManager::new(&db_pool);
    Migrator::up(&db_pool, None).await.unwrap();
    assert!(schema_manager.has_table("question").await.unwrap());
    assert!(schema_manager.has_table("answer").await.unwrap());
    HttpServer::new(move || 
            App::new()
            .wrap(auth.to_owned())
            .app_data(Data::new(db_pool.to_owned()))
            .service(routes::root)
            .wrap(Logger::default())
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
