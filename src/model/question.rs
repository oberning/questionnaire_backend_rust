//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "question")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub text: String,
    pub max_score: i32,
    pub answer_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::answer::Entity",
        from = "Column::AnswerId",
        to = "super::answer::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Answer,
    #[sea_orm(has_many = "super::questionnaire::Entity")]
    Questionnaire,
}

impl Related<super::answer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Answer.def()
    }
}

impl Related<super::questionnaire::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questionnaire.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
