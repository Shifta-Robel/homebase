use sea_orm::{error::DbErr, EntityTrait};
use crate::config::AppConfig;
use crate::models::quicklink::quicklink::{Model, Entity as Quicklink};

    // let url = "sqlite://./src/db/quicklinks.sqlite";
pub async fn get(config: &AppConfig) -> Result<Vec<Model>, DbErr> {
    // let path = "sqlite://".to_string() + &config.quicklinks_db_path.clone()[..];
    let path = config.quicklinks_db_path.clone().unwrap_or(Model::get_default_path());
    let path = ["sqlite://", path.as_str()].concat();
    let conn = crate::db::connect(&path).await?;
    Quicklink::find().all(&conn).await
}
