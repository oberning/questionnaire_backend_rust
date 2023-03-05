use crate::model::questions_answers::{Questions, ToJson};
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(Questions::default().to_json())
}
