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

impl<'a> Default for Questions {
    fn default() -> Self {
        Questions {
            questions: vec![
                Question {
                    id: 1,
                    question: "Was ist die erste Frage".to_owned(),
                    max_score: 2,
                    answers: vec![
                        Answer { id: 1, answer: "Antwort 1".to_owned(), is_correct: true },
                        Answer { id: 2, answer: "Antwort 2".to_owned(), is_correct: false },
                        Answer { id: 3, answer: "Antwort 3".to_owned(), is_correct: false },
                        Answer { id: 4, answer: "Antwort 4".to_owned(), is_correct: true }
                    ],
                },
                Question {
                    id: 2,
                    question: "Wie fühlst du dich?".to_owned(),
                    max_score: 3,
                    answers: vec![
                        Answer { id: 5, answer: "beschissen".to_owned(), is_correct: false },
                        Answer { id: 6, answer: "gut".to_owned(), is_correct: true },
                        Answer { id: 7, answer: "kann nicht klagen".to_owned(), is_correct: true },
                        Answer { id: 8, answer: "geht so".to_owned(), is_correct: false }
                    ],
                },
            ],
        }
    }
}
