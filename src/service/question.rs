use crate::model::{answer, prelude::*, question};
use log::debug;
use macros::ToJson;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToJson, PartialEq)]
struct QuestionDto {
    id: i32,
    text: String,
    max_score: i32,
    answers: Vec<AnswerDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AnswerDto {
    id: i32,
    text: String,
    is_correct: bool,
}

fn map_question(
    question: Option<&(question::Model, Vec<answer::Model>)>,
) -> Result<QuestionDto, ()> {
    match question {
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

async fn question_find(question_id: i32, db: &DatabaseConnection) -> Result<QuestionDto, ()> {
    let questions = Question::find_by_id(question_id)
        .find_with_related(Answer)
        .all(db)
        .await
        .unwrap();
    map_question(questions.first())
}

pub async fn question(question_id: i32, db: &DatabaseConnection) -> Result<String, ()> {
    let question = match question_find(question_id, db).await {
        Ok(question) => question.to_json(),
        Err(e) => return Err(e),
    };
    debug!("Question: {}", question);
    Ok(question)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let question_model = question::Model {
            id: 5000,
            text: "Question_1".to_string(),
            max_score: 100,
            questionnaire_id: 815,
        };
        let answer_model = vec![
            answer::Model {
                id: 1,
                text: "Answer_1".to_string(),
                is_correct: false,
                question_id: 1,
            },
            answer::Model {
                id: 2,
                text: "Answer_2".to_string(),
                is_correct: true,
                question_id: 1,
            },
            answer::Model {
                id: 3,
                text: "Answer_3".to_string(),
                is_correct: false,
                question_id: 1,
            },
        ];
        let result = map_question(Some(&(question_model, answer_model))).unwrap_or(QuestionDto {
            id: -1,
            text: "".to_string(),
            max_score: -1,
            answers: vec![],
        });
        let expected = QuestionDto {
            id: 5000,
            text: "Question_1".to_string(),
            max_score: 100,
            answers: vec![
                AnswerDto {
                    id: 1,
                    text: "Answer_1".to_string(),
                    is_correct: false,
                },
                AnswerDto {
                    id: 2,
                    text: "Answer_2".to_string(),
                    is_correct: true,
                },
                AnswerDto {
                    id: 3,
                    text: "Answer_3".to_string(),
                    is_correct: false,
                },
            ],
        };
        assert_eq!(result, expected);
    }
}
