use sea_orm_migration::prelude::*;

mod m20230312_000001_create_questionnaire_table;
mod m20230312_000002_create_question_table;
mod m20230312_000003_create_answer_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230312_000001_create_questionnaire_table::Migration),
            Box::new(m20230312_000002_create_question_table::Migration),
            Box::new(m20230312_000003_create_answer_table::Migration),
        ]
    }
}