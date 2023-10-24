use sea_orm::{error::DbErr, EntityTrait};
use crate::config::AppConfig;
use crate::models::quicklink::quicklink::{Model, Entity as Quicklink};

pub async fn get(config: &AppConfig) -> Result<Vec<Model>, DbErr> {
    let path = &config.files.quicklinks_db_path;
    // let path = ["sqlite://", path.as_str()].concat();
    let path = format!("sqlite://{}",path.display());
    let conn = crate::db::connect(&path).await?;
    Quicklink::find().all(&conn).await
}
