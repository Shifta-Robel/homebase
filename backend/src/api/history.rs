use actix_web::{
    get, HttpResponse
};
use rusqlite::Result;
use crate::errors::ServerError;
// use crate::models::HistoryItem;

use crate::services::history::get_history;

#[get("/history")]
pub async fn history() -> Result<HttpResponse, ServerError>{
    let history = get_history()?;
    let response = serde_json::to_string(&history).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
