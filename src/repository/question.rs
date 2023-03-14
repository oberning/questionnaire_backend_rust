use crate::model::{prelude::*};
use log::info;
use sea_orm::prelude::*;

pub async fn test(db: &DatabaseConnection) {
    let question = Question::find_by_id(2)
        .find_with_related(Answer)
        .all(db)
        .await;
    let models = question.unwrap();
    info!("Result: {:?}", models);
    todo!("Pack the Model tuple into a struct");
}