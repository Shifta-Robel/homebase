use actix_web::{
    web, HttpResponse
};
use rusqlite::Result;
use crate::config::AppConfig;
use crate::errors::ServerError;
// use crate::models::history_item::HistoryItem;
use crate::services::history::get_history;

pub async fn history(config: web::Data<AppConfig>) -> Result<HttpResponse, ServerError>{
    let path = &config.files.history_db_path;
    let history = get_history(path.to_path_buf())?;
    let response = serde_json::to_string(&history).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
