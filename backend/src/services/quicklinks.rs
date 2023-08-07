use sea_orm::EntityTrait;
use sea_orm::error::DbErr;
use crate::models::quicklink::quicklink::Model;
use sea_orm::DatabaseConnection;

use super::super::models::quicklink::quicklink::Entity as Quicklink;


pub async fn get(config: DatabaseConnection , id: i32) -> Result<Option<Model>, DbErr> {
    let conn = crate::db::connect(config.quicklinks_path)?
    Quicklink::find_by_id(id).one(&conn).await
}
