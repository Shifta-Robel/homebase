use sea_orm::{error::DbErr, EntityTrait};
use crate::config::AppConfig;
// use crate::models::quicklink::quicklink::{Model, Entity as Quicklink};
use crate::models::reading_list::reading_list::{Model, Entity as ReadingList};

pub async fn get(config: &AppConfig) -> Result<Vec<Model>, DbErr> {
    let path = config.reading_list_db_path.clone().unwrap_or(Model::get_default_path());
    let path = ["sqlite://", path.as_str()].concat();
    let conn = crate::db::connect(&path).await?;
    ReadingList::find().all(&conn).await
}
