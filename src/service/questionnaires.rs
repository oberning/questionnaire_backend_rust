use log;
use macros::ToJson;
use serde::{Serialize, Deserialize};
use sqlx::{SqlitePool, FromRow};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToJson)]
struct Questionnaires {
    pub questionnaires: Vec<Questionnaire>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
struct Questionnaire {
    pub id: i32,
    pub name: String,
}

pub async fn questionnaires(db: &SqlitePool) -> Result<String, sqlx::Error> {
    let questionnaires = sqlx::query_as::<_, Questionnaire>("SELECT * FROM questionnaire")
        .fetch_all(db)
        .await?;
    log::debug!("Questionnaires: {:?}", questionnaires);
    let result = Questionnaires { questionnaires };
    Ok(result.to_json())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(true, true);
    }
}
