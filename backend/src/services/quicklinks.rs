use sea_orm::{error::DbErr, EntityTrait};
use crate::config::AppConfig;
use crate::models::quicklink::quicklink::{Model, Entity as Quicklink};

pub async fn get(config: &AppConfig<'_>) -> Result<Vec<Model>, DbErr> {
    let conn = crate::db::connect(config.quicklinks_db_path).await?;
    Quicklink::find().all(&conn).await
}
