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

pub async fn question(question_id: i32, db: &DatabaseConnection) -> Result<String, ()> {
    let question = match question_dto(question_id, db).await {
        Ok(question) => question.to_json(),
        Err(e) => return Err(e)
    };
    debug!("Question: {}", question);
    Ok(question)
}

async fn question_dto(question_id: i32, db: &DatabaseConnection) -> Result<QuestionDto, ()> {
    let questions = Question::find_by_id(question_id)
        .find_with_related(Answer)
        .all(db)
        .await
        .unwrap();
    match questions.first() {
        Some((question, answers)) => Ok(QuestionDto {
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
        }),
        None => return Err(()),
    }
    
}
