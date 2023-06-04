use std::collections::HashMap;

use macros::ToJson;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToJson)]
struct QuestionsAnswers {
    questionnaire_id: i32,
    questions: Vec<Question>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Question {
    id: i32,
    text: String,
    max_score: i32,
    answers: Vec<Answer>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Answer {
    id: i32,
    text: String,
    is_correct: bool,
}

const SQL: &str = r#"
    SELECT
        q.id as question_id,
        q.text as question_text,
        q.max_score,
        a.id as answer_id,
        a."text" as answer_text,
        a.is_correct
    FROM
        question q
    JOIN answer a ON
        q.id = a.question_id
    WHERE
        q.questionnaire_id = ?
    ORDER BY q.id, a.id
"#;

pub async fn questionnaire(id: i32, db: &SqlitePool) -> Result<String, sqlx::Error> {
    let rows = sqlx::query(SQL).bind(id).fetch_all(db).await?;
    let mut questions_map: HashMap<i32, Question> = HashMap::new();
    for row in rows {
        let question_id: i32 = row.get("question_id");
        let question_text: String = row.get("question_text");
        let max_score: i32 = row.get("max_score");
        let answer_id: i32 = row.get("answer_id");
        let answer_text: String = row.get("answer_text");
        let is_correct: bool = row.get("is_correct");
        let questions = questions_map.entry(question_id).or_insert(Question {
            id: question_id,
            text: question_text,
            max_score,
            answers: Vec::new(),
        });
        questions.answers.push(Answer {
            id: answer_id,
            text: answer_text,
            is_correct,
        });
    }
    let mut questions = questions_map.values().cloned().collect::<Vec<Question>>();
    questions.sort_by_key(|question| question.id);
    let result = QuestionsAnswers {
        questionnaire_id: id,
        questions,
    };
    log::debug!("Questions and Answers: {:?}", result);
    Ok(result.to_json())
}
