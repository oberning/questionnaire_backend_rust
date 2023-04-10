use serde::{Deserialize, Serialize};
use macros::ToJson;

#[derive(Serialize, Deserialize, Debug, ToJson)]
pub struct Questions {
    questions: Vec<Question>
}

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    id: i32,
    question: String,
    max_score: i32,
    answers: Vec<Answer>
}

#[derive(Serialize, Deserialize, Debug)]
struct Answer {
    id: i32,
    answer: String,
    is_correct: bool
}
