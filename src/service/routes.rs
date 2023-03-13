use crate::model::questions_answers::{Questions, ToJson};
use actix_web::{get, HttpResponse, Responder, HttpRequest, web::Data};
use sea_orm::DatabaseConnection;

// #[get("/")]
// async fn root(req: HttpRequest, db_pool: Data<DatabaseConnection>) -> impl Responder {
//     HttpResponse::Ok().body(Questions::default().to_json())
// }
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(Questions::default().to_json())
}
