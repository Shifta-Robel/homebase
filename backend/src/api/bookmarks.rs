use actix_web::{get, HttpResponse};
use rusqlite::Result;
use crate::errors::ServerError;
// use crate::models::Bookmark;

use crate::services::bookmarks::get_bookmarks;

#[get("/bookmarks")]
async fn bookmarks() -> Result<HttpResponse, ServerError> {
    let bookmark_vector = get_bookmarks()?;
    let response = serde_json::to_string(&bookmark_vector).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
