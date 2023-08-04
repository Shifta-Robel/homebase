use actix_web::{get, HttpResponse};
use rusqlite::Result;
use crate::errors::ServerError;
// use crate::models::Quickmark;
use crate::services::quickmarks::get_quickmarks;

#[get("/quickmarks")]
async fn quickmarks() -> Result<HttpResponse, ServerError> {
    let quickmark_vector = get_quickmarks()?;
    let response = serde_json::to_string(&quickmark_vector).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
