use actix_web::{web, HttpResponse};

use crate::{errors::ServerError, models::commands::Command};

pub async fn run_command(data: web::Json<Command>) -> Result<HttpResponse, ServerError> {
    let cmd = &data.command;
    crate::services::run_command::run_command(cmd.to_string());
    let output = "succesfully ran command".to_string();
    let response = serde_json::to_string(&output).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response),
    )
}
