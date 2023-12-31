//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "share_content")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub share_config_id: u64,
    pub doc_id: u64,
    pub parent_id: u64,
    pub is_share_folder: i8,
    pub is_deleted: i8,
    pub gmt_create: Option<DateTime>,
    pub gmt_modified: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
