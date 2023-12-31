//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "debug_script")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(column_name = "NAME")]
    pub name: String,
    pub description: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub r#type: i8,
    pub scope: i8,
    pub ref_id: u64,
    pub creator_name: String,
    pub enabled: i8,
    pub is_deleted: i8,
    pub gmt_create: Option<DateTime>,
    pub gmt_modified: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
