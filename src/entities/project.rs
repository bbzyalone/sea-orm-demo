//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub space_id: i64,
    pub is_private: i8,
    pub creator_id: u64,
    pub creator_name: String,
    pub modifier_id: u64,
    pub modifier_name: String,
    pub order_index: i32,
    pub is_deleted: i8,
    pub gmt_create: Option<DateTime>,
    pub gmt_modified: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
