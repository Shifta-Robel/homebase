use actix_web::{web, HttpResponse};
use rusqlite::Result;
use crate::config::AppConfig;
use crate::errors::ServerError;
use crate::models::quickmark::Quickmark;
use crate::services::quickmarks::get_quickmarks;

pub async fn quickmarks(config: web::Data<AppConfig>) -> Result<HttpResponse, ServerError> {
    let path = config.quickmark_file_path.clone().unwrap_or(Quickmark::get_default_path());
    let quickmark_vector = get_quickmarks(&path)?;
    let response = serde_json::to_string(&quickmark_vector).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
