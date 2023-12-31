//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "share_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub r#type: i8,
    pub password: String,
    pub status: i8,
    pub module_id: i64,
    pub is_all: i8,
    pub is_deleted: i8,
    pub remark: String,
    pub creator_name: String,
    pub is_show_debug: i8,
    pub is_all_selected_debug: i8,
    pub gmt_create: Option<DateTime>,
    pub gmt_modified: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
