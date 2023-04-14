use crate::service::{question, questionnaires, questionnaire};
use actix_web::{get, HttpResponse, Responder, HttpRequest, web::Data, web::Path};
use sea_orm::{DatabaseConnection};

#[get("/question/{id}")]
async fn get_question(path: Path<i32>, _req: HttpRequest, db_pool: Data<DatabaseConnection>) -> impl Responder {
    match question::question(path.into_inner(), &db_pool).await {
        Ok(question) => HttpResponse::Ok().body(question),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[get("/questionnaires")]
async fn get_questionnaires(db_pool: Data<DatabaseConnection>) -> impl Responder {
    match questionnaires::questionnaires(&db_pool).await {
        Ok(questionnaires) => HttpResponse::Ok().body(questionnaires),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[get("/questionnaire/{id}")]
async fn get_questionnaire(path: Path<i32>, _req: HttpRequest, db_pool: Data<DatabaseConnection>) -> impl Responder {
    match questionnaire::questionnaire(path.into_inner(), &db_pool).await {
        Ok(questionnaire) => HttpResponse::Ok().body(questionnaire),
        Err(_) => HttpResponse::NotFound().finish()
    }
}