//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "system_i18n_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub lang: String,
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub is_deleted: i8,
    pub gmt_create: Option<DateTime>,
    pub gmt_modified: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}