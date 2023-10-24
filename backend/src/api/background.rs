use actix_web::{web, HttpResponse};
use crate::{config::AppConfig, errors::ServerError};

pub async fn get_background(config: web::Data<AppConfig>) -> Result<HttpResponse, ServerError>{
    let response = serde_json::to_string(&config.background).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}
