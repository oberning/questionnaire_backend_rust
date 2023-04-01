use sea_orm_migration::prelude::*;

use super::m20230312_000001_create_questionnaire_table::Questionnaire;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230312_000002_create_question_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Question::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::Text).string().not_null())
                    .col(ColumnDef::new(Question::MaxScore).integer().not_null())
                    .col(ColumnDef::new(Question::QuestionnaireId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-question-questionnaire_id")
                            .from(Question::Table, Question::QuestionnaireId)
                            .to(Questionnaire::Table, Questionnaire::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Question {
    Table,
    Id,
    Text,
    MaxScore,
    QuestionnaireId,
}
