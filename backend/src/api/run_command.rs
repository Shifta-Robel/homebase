use actix_web::{web, HttpResponse};

use crate::{services, errors::ServerError, models::commands::Command, config::AppConfig};

pub async fn run_command(_config: web::Data<AppConfig>, data: web::Json<Command>) -> Result<HttpResponse, ServerError> {
    // todo!("get detached or child options from command and run command based on that");
    let cmd = &data.command;
    services::run_command::run_command(cmd.to_string());
    let output = "succesfully ran command".to_string();
    let response = serde_json::to_string(&output).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response),
    )
}
