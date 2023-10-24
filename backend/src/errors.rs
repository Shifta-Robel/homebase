use actix_web::{
    error, HttpResponse,
    http::{header::ContentType, StatusCode},
};
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    DbError(rusqlite::Error),
    SerializeError(serde_json::Error),
    IOError(std::io::Error),
    SqliteError(sea_orm::DbErr),
    FailedCommandExecution(),
    ConfigurationAccessError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::SerializeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::SqliteError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::FailedCommandExecution() => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::ConfigurationAccessError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl std::convert::From<sea_orm::DbErr> for ServerError {
    fn from(err: sea_orm::DbErr) -> Self {
        ServerError::SqliteError(err)
    }
}

#[derive(Debug)]
pub enum UserError {
    InvalidConfigurationError(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidConfigurationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).body(self.to_string())
    }
}
