use crate::service::question;
use actix_web::{get, HttpResponse, Responder, HttpRequest, web::Data, web::Path};
use sea_orm::{DatabaseConnection};

#[get("/question/{id}")]
async fn root(path: Path<i32>, _req: HttpRequest, db_pool: Data<DatabaseConnection>) -> impl Responder {
    match question::question(path.into_inner(), &db_pool).await {
        Ok(question) => HttpResponse::Ok().body(question),
        Err(_) => HttpResponse::NotFound().finish()
    }
}
