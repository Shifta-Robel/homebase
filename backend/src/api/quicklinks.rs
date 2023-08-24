use actix_web::{web, HttpResponse};
use rusqlite::Result;
use crate::config::AppConfig;
use crate::errors::ServerError;
// use crate::models::Bookmark;
use crate::services::quicklinks;

pub async fn quicklink(config: web::Data<AppConfig>) -> Result<HttpResponse, ServerError> {
    let model = quicklinks::get(config.get_ref()).await?;
    let response = serde_json::to_string(&model).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
