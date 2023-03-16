use crate::service::question;
use actix_web::{get, HttpResponse, Responder, HttpRequest, web::Data, web::Path};
use sea_orm::DatabaseConnection;

#[get("/questions/{id}")]
async fn root(path: Path<i32>, _req: HttpRequest, db_pool: Data<DatabaseConnection>) -> impl Responder {
    let question = question::question(path.into_inner(), &db_pool).await;
    HttpResponse::Ok().body(question)
}
