mod auth;
mod service;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use service::{database, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    let db_pool = database::build_connection_pool().await.unwrap();
    HttpServer::new(move || {
        let _auth = HttpAuthentication::bearer(auth::validator);
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            // .wrap(_auth)
            .app_data(Data::new(db_pool.to_owned()))
            .service(routes::get_questionnaires)
            .service(routes::get_questions_answers)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
