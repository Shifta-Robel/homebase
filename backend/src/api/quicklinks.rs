use actix_web::{get, web, HttpResponse};
use rusqlite::Result;
use crate::config::AppConfig;
use crate::errors::ServerError;
// use crate::models::Bookmark;

use crate::services::quicklinks::get;
use crate::db::connect;

#[get("/quicklink")]
async fn quicklink(config: web::Data<&AppConfig>) -> Result<HttpResponse, ServerError> {
    let conn = connect().await?;
    let model = get(conn, 1).await?.unwrap();
    let response = serde_json::to_string(&model.url).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
