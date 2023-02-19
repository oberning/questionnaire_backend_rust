use crate::model::questions_answers::print_question;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(print_question())
}
