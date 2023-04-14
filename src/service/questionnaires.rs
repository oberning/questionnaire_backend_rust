use crate::model::{prelude::*, questionnaire};
use log;
use macros::ToJson;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToJson, PartialEq)]
struct QuestionnairesDto {
    questionnaires: Vec<QuestionnaireDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct QuestionnaireDto {
    id: i32,
    name: String,
}

fn map_questionnaires(questionnaires: Vec<questionnaire::Model>) -> QuestionnairesDto {
    QuestionnairesDto {
        questionnaires: questionnaires
            .into_iter()
            .map(|questionnaire| QuestionnaireDto {
                id: questionnaire.id,
                name: questionnaire.name,
            })
            .collect(),
    }
}

async fn questionnaires_find(db: &DatabaseConnection) -> QuestionnairesDto {
    let questionnaires = Questionnaire::find().all(db).await.unwrap();
    log::debug!("Questionnaires: {:?}", questionnaires);
    map_questionnaires(questionnaires)
}

pub async fn questionnaires(db: &DatabaseConnection) -> Result<String, ()> {
    let questionnaires = questionnaires_find(db).await;
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
