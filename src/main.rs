mod model;
mod service;
use actix_web::{App, HttpServer};
use service::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::root))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
