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
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[derive(Debug)]
pub enum UserError {

}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for UserError {
    // fn status_code(&self) -> StatusCode {
    //     match self {
    //     }
    // }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).body(self.to_string())
    }
}
