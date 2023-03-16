use crate::model::prelude::*;
use log::debug;
use macros::ToJson;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToJson)]
struct QuestionDto {
    id: i32,
    text: String,
    max_score: i32,
    answers: Vec<AnswerDto>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnswerDto {
    id: i32,
    text: String,
    is_correct: bool,
}

pub async fn question(question_id: i32, db: &DatabaseConnection) -> String {
    let question = question_dto(question_id, db).await.to_json();
    debug!("Question: {}", question);
    question
}

async fn question_dto(question_id: i32, db: &DatabaseConnection) -> QuestionDto {
    let questions = Question::find_by_id(question_id)
        .find_with_related(Answer)
        .all(db)
        .await
        .unwrap();
    let (question, answers) = questions.first().unwrap();
    QuestionDto {
        id: question.id,
        text: question.text.to_owned(),
        max_score: question.max_score,
        answers: answers
            .into_iter()
            .map(|answer| AnswerDto {
                id: answer.id,
                text: answer.text.to_owned(),
                is_correct: answer.is_correct,
            })
            .collect(),
    }
}
