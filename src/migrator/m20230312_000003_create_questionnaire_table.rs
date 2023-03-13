use sea_orm_migration::prelude::*;
use super::m20230312_000002_create_question_table::Question;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230312_000003_create_questionnaire_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Questionnaire::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Questionnaire::Id)
                            .integer()
                            .not_null()
                    )
                    .col(ColumnDef::new(Questionnaire::Name).string().not_null())
                    .col(ColumnDef::new(Questionnaire::QuestionId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-questionnaire")
                            .col(Questionnaire::Id)
                            .col(Questionnaire::QuestionId)
                            .primary()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-questionnaire-question_id")
                            .from(Questionnaire::Table, Questionnaire::QuestionId)
                            .to(Question::Table, Question::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Questionnaire::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Questionnaire {
    Table,
    Id,
    Name,
    QuestionId,
}
