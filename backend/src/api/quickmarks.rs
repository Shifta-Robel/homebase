use actix_web::{web, HttpResponse, HttpRequest};
use rusqlite::Result;
use crate::config::AppConfig;
use crate::errors::ServerError;
// use crate::models::Quickmark;
use crate::services::quickmarks::get_quickmarks;

pub async fn quickmarks(_req: HttpRequest, config: web::Data<AppConfig>) -> Result<HttpResponse, ServerError> {
    let quickmark_vector = get_quickmarks(&config)?;
    let response = serde_json::to_string(&quickmark_vector).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
