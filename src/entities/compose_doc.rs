//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "compose_doc")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub doc_id: u64,
    pub project_id: u64,
    pub is_folder: i8,
    pub folder_name: String,
    pub parent_id: i64,
    pub origin: String,
    pub is_deleted: i8,
    pub creator: String,
    pub order_index: u32,
    pub gmt_create: Option<DateTime>,
    pub gmt_modified: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
