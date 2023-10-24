use sea_orm::{error::DbErr, EntityTrait};
use crate::config::AppConfig;
// use crate::models::quicklink::quicklink::{Model, Entity as Quicklink};
use crate::models::reading_list::reading_list::{Model, Entity as ReadingList};

pub async fn get(config: &AppConfig) -> Result<Vec<Model>, DbErr> {
    let path = &config.files.reading_list_db_path;
    // let path = ["sqlite://", path.into_string()].concat();
    let path = format!("sqlite://{}",path.display());
    let conn = crate::db::connect(&path).await?;
    ReadingList::find().all(&conn).await
}
