use sea_orm::{DatabaseConnection, ModelTrait};
use sea_orm::{error::DbErr, EntityTrait};
use crate::{db, config::AppConfig};
// use crate::models::quicklink::quicklink::{Model, Entity as Quicklink};
// use crate::models::reading_list::reading_list::{Model, Entity as ReadingList};
use crate::models::shortcuts::{
    shortcut::{Model, Entity as Shortcut},
    commands::{Model as CommandModel, Entity as Command},
    urls::{Model as UrlModel, Entity as Url},
};

pub async fn get(config: &AppConfig) -> Result<Vec<Model>, DbErr> {
    // let path = config.shortcuts_db_path.clone().unwrap_or(Model::get_default_path());
    // let path = ["sqlite://", path.as_str()].concat();
    // let conn = db::connect(&path).await?;
    println!("inside get");
    let conn = connect(config).await?;
    Shortcut::find().all(&conn).await
}

// pub async fn get_by_id(config: &AppConfig) -> Result<Vec<Model>, DbErr> {
//     let conn = connect(config).await?;
//     
//
// }

pub async fn get_commands_by_shortcut_id(config: &AppConfig, id: u32) -> Result<Vec<CommandModel>, DbErr> {
    let conn = connect(config).await?;
    let shortcut = Shortcut::find_by_id(id).one(&conn).await?.unwrap();
    shortcut.find_related(Command).all(&conn).await
}

async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let path = config.shortcuts_db_path.clone().unwrap_or(Model::get_default_path());
    let path = ["sqlite://", path.as_str()].concat();
    db::connect(&path).await
}
