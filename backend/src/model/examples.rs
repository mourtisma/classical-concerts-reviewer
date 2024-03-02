//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "examples")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "custom(\"citext\")"
    )]
    pub id: String,
    #[sea_orm(column_type = "custom(\"citext\")")]
    pub name: String,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
