use serde::{Deserialize, Serialize};
use macros::ToJson;

#[derive(Serialize, Deserialize, Debug, ToJson)]
struct Questions<'a> {
    #[serde(borrow)]
    questions: Vec<Question<'a>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Question<'a> {
    question: &'a str,
    answers: Vec<(&'a str, bool)>,
}

impl<'a> Default for Questions<'a> {
    fn default() -> Self {
        Questions {
            questions: vec![
                Question {
                    question: "Was ist die erste Frage",
                    answers: vec![
                        ("Antwort 1", true),
                        ("Antwort 2", false),
                        ("Antwort 3", false),
                        ("Antwort 4", true),
                    ],
                },
                Question {
                    question: "Wie fühlst du dich?",
                    answers: vec![
                        ("beschissen", false),
                        ("gut", true),
                        ("kann nicht klagen", true),
                        ("geht so", false),
                    ],
                },
            ],
        }
    }
}


pub fn print_question() -> String {
    let questions = Questions::default();
    questions.to_json()
}