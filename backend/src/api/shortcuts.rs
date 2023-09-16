use actix_web::{web, HttpResponse};
use rusqlite::Result;
use crate::config::AppConfig;
use crate::errors::ServerError;
// use crate::models::Bookmark;

use crate::services::shortcuts;

pub async fn shortcuts(config: web::Data<AppConfig>) -> Result<HttpResponse, ServerError> {
    let model = shortcuts::get(config.get_ref()).await?;
    let response = serde_json::to_string(&model).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}

pub async fn shortcuts_command_by_id(config: web::Data<AppConfig>, data: web::Path<u32>) -> Result<HttpResponse, ServerError> {
    println!("got a data of {}", data);
    let model = shortcuts::get_commands_by_shortcut_id(config.get_ref(), data.into_inner()).await?;
    let response = serde_json::to_string(&model).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
