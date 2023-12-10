#![allow(unused)]

mod entities;

use futures::executor::block_on;
use sea_orm::ColumnType::DateTime;
use log::log;
use crate::entities::{prelude::*, *};
use sea_orm::*;

const DATABASE_URL: &str = "mysql://root:wkqdmm@192.168.6.173:3306";
const DB_NAME: &str = "sea_orm_test";


//TODO 通过mod导入
//获取数据库连接池
async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(&url).await?;
    Ok(db)
}


//insert
async fn run() -> Result<(), DbErr> {
    println!("添加数据");
    let user_info_new = user_info::ActiveModel {
        //id: ActiveValue::Set(999),
        //username: ActiveValue::Set(n.to_string()),
        password: ActiveValue::Set("1237456".to_owned()),
        nickname: ActiveValue::Set("bbzyalone".to_owned()),
        is_super_admin: ActiveValue::Set(0),
        source: ActiveValue::Set("麻了".to_owned()),
        email: ActiveValue::Set("mmale".to_owned()),
        status: ActiveValue::Set(0),
        is_deleted: ActiveValue::Set(0),
        //gmt_create:ActiveValue::Set(DateTime.),
        ..Default::default()
    };
    let db = get_db_connection().await.unwrap();
    let res = UserInfo::insert(user_info_new).exec(&db).await?;
    Ok(())
}

//delete
async fn del() -> Result<(), DbErr> {
    println!("删除数据");
    let user_info = user_info::ActiveModel {
        id: ActiveValue::Set(999),
        ..Default::default()
    };
    let db = get_db_connection().await.unwrap();
    user_info.delete(&db).await?;
    Ok(())
}


//update
async fn update() -> Result<(), DbErr> {
    println!("修改数据");
    let user_info_up = user_info::ActiveModel {
        id: ActiveValue::Set(1204),
        //username: ActiveValue::Set(n.to_string()),
        password: ActiveValue::Set("1237456".to_owned()),
        nickname: ActiveValue::Set("我就想不通了".to_owned()),
        is_super_admin: ActiveValue::Set(0),
        source: ActiveValue::Set("我就想不通了".to_owned()),
        email: ActiveValue::Set("我就想不通了".to_owned()),
        status: ActiveValue::Set(0),
        is_deleted: ActiveValue::Set(0),
        ..Default::default()
    };
    let db = get_db_connection().await.unwrap();
    user_info_up.update(&db).await?;
    Ok(())
}

//查询数据
async fn select() -> Result<(), DbErr> {
    let db = get_db_connection().await.unwrap();
    println!("查询数据");
    let doc_infos: Vec<doc_info::Model> = DocInfo::find().all(&db).await?;
    for x in doc_infos {
        println!("> {}", x.name);
    };
    Ok(())
}

fn main() {
    // if let Err(err) = block_on(run()) {
    //     panic!("{}", err);
    // }

    // if let Err(err) = block_on(del()) {
    //     panic!("{}", err);
    // }

    // if let Err(err) = block_on(update()) {
    //     panic!("{}", err);
    // }

    if let Err(err) = block_on(select()) {
        panic!("{}", err);
    }
}