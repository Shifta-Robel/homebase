use actix_web::{web, HttpResponse, HttpRequest};
use rusqlite::Result;
use crate::config::AppConfig;
use crate::errors::ServerError;
use crate::models::bookmark::Bookmark;
use crate::services::bookmarks::get_bookmarks;

pub async fn bookmarks(_req: HttpRequest, config: web::Data<AppConfig>) -> Result<HttpResponse, ServerError> {
    let path = config.bookmarks_file_path.clone().unwrap_or(Bookmark::get_default_path());
    let bookmark_vector = get_bookmarks(&path)?;
    let response = serde_json::to_string(&bookmark_vector).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
