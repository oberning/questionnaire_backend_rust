use crate::service::{questionnaires, questions_answers};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use sqlx::SqlitePool;

#[get("/questionnaires")]
async fn get_questionnaires(db_pool: Data<SqlitePool>) -> HttpResponse {
    match questionnaires::questionnaires(&db_pool).await {
        Ok(questionnaires) => HttpResponse::Ok().body(questionnaires),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/questionnaire/{id}")]
async fn get_questions_answers(path: Path<i32>, db_pool: Data<SqlitePool>) -> HttpResponse {
    match questions_answers::questionnaire(path.into_inner(), &db_pool).await {
        Ok(questions_answers) => HttpResponse::Ok().body(questions_answers),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
