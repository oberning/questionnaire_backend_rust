use sea_orm_migration::prelude::*;

use super::m20230312_000002_create_question_table::Question;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230312_000001_create_answer_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Answer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Answer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Answer::Text).string().not_null())
                    .col(ColumnDef::new(Answer::IsCorrect).boolean().not_null())
                    .col(ColumnDef::new(Answer::QuestionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-answer-question_id")
                            .from(Answer::Table, Answer::QuestionId)
                            .to(Question::Table, Question::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Answer::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Answer {
    Table,
    Id,
    Text,
    IsCorrect,
    QuestionId,
}
