use serde::{Deserialize, Serialize};
use macros::ToJson;

#[derive(Serialize, Deserialize, Debug, ToJson)]
pub struct Questions<'a> {
    #[serde(borrow)]
    questions: Vec<Question<'a>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Question<'a> {
    id: i32,
    question: &'a str,
    max_score: i32,
    answers: Vec<Answer<'a>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Answer<'a> {
    id: i32,
    answer: &'a str,
    is_correct: bool
}

impl<'a> Default for Questions<'a> {
    fn default() -> Self {
        Questions {
            questions: vec![
                Question {
                    id: 1,
                    question: "Was ist die erste Frage",
                    max_score: 2,
                    answers: vec![
                        Answer { id: 1, answer: "Antwort 1", is_correct: true },
                        Answer { id: 2, answer: "Antwort 2", is_correct: false },
                        Answer { id: 3, answer: "Antwort 3", is_correct: false },
                        Answer { id: 4, answer: "Antwort 4", is_correct: true }
                    ],
                },
                Question {
                    id: 2,
                    question: "Wie fühlst du dich?",
                    max_score: 3,
                    answers: vec![
                        Answer { id: 5, answer: "beschissen", is_correct: false },
                        Answer { id: 6, answer: "gut", is_correct: true },
                        Answer { id: 7, answer: "kann nicht klagen", is_correct: true },
                        Answer { id: 8, answer: "geht so", is_correct: false }
                    ],
                },
            ],
        }
    }
}
