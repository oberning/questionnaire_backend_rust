use crate::model::{prelude::*, question, questionnaire};
use log;
use macros::ToJson;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToJson, PartialEq)]
struct QuestionnaireDto {
    id: i32,
    name: String,
    questions: Vec<QuestionDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct QuestionDto {
    id: i32,
    text: String,
    max_score: i32,
}

fn map_questionnaire(
    questionnaire: &(questionnaire::Model, Vec<question::Model>),
) -> QuestionnaireDto {
    QuestionnaireDto {
        id: questionnaire.0.id,
        name: questionnaire.0.name.to_owned(),
        questions: questionnaire
            .1
            .to_owned()
            .into_iter()
            .map(|e| QuestionDto {
                id: e.id,
                text: e.text,
                max_score: e.max_score,
            })
            .collect(),
    }
}

async fn questionnaire_find(questionnaire_id: i32, db: &DatabaseConnection) -> QuestionnaireDto {
    let questionnaire = Questionnaire::find_by_id(questionnaire_id)
        .find_with_related(Question)
        .all(db)
        .await
        .unwrap();
    log::debug!("Questionnaire: {:?}", questionnaire);
    map_questionnaire(questionnaire.first().unwrap())
}

pub async fn questionnaire(questionnaire_id: i32, db: &DatabaseConnection) -> Result<String, ()> {
    let questionnaires = questionnaire_find(questionnaire_id, db).await;
    log::debug!("Questionnaires: {:?}", questionnaires);
    Ok(questionnaires.to_json())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(true, true);
    }
}
