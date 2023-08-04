use actix_web::{
    get, post, error, App, HttpResponse, HttpServer,
    http::{header::ContentType, StatusCode},
    middleware::Logger,
};
use actix_cors::Cors;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, fs};

#[derive(Serialize)]
struct HistoryItem {
    url: String,
    title: String,
    time: i64
}

#[derive(Serialize)]
struct Bookmark {
    url: String,
    title: String,
}

#[derive(Serialize)]
struct Quickmark {
    shortcut: String,
    url: String,
}

#[derive(Debug)]
enum ServerError {
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
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).body(self.to_string())
    }
}

// #[derive(Debug)]
// enum UserError {
// }
//
// impl fmt::Display for UserError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }
//
// impl error::ResponseError for UserError {
//     fn status_code(&self) -> StatusCode {
//         match self {
//         }
//     }
//     fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
//         HttpResponse::build(self.status_code()).insert_header(ContentType::json()).body(self.to_string())
//     }
// }

#[get("/history")]
async fn history() -> Result<HttpResponse, ServerError>{
    let conn = Connection::open("/home/robel/.local/share/qutebrowser/history.sqlite").map_err(ServerError::DbError)?;
    let mut stmt = conn.prepare("SELECT url, title, atime FROM History")
        .map_err(ServerError::DbError)?;
    let history = stmt.query_map([], |row|
        Ok(
            HistoryItem {
                url: row.get(0)?,
                title: row.get(1)?,
                time: row.get(2)?
            }
        )
    ).map_err(ServerError::DbError)?;
    let history: Vec<HistoryItem> = history.map(|item| item.unwrap()).collect();
    let response = serde_json::to_string(&history).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}

#[get("/bookmarks")]
async fn bookmarks() -> Result<HttpResponse, ServerError> {
    let contents = fs::read_to_string("/home/robel/.config/qutebrowser/bookmarks/urls").map_err(ServerError::IOError)?;
    let lines = contents.lines();
    let bookmark_vector: Vec<_> = lines.map(|line| {
        let idx = line.find(' ').unwrap();
        return Bookmark {
            url: line[0..idx].to_string(),
            title: line[idx+1..].to_string(),
        }
    }).collect();
    let response = serde_json::to_string(&bookmark_vector).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}

#[get("/quickmarks")]
async fn quickmarks() -> Result<HttpResponse, ServerError> {
    let contents = fs::read_to_string("/ome/robel/.config/qutebrowser/quickmarks").map_err(ServerError::IOError)?;
    let lines = contents.lines();
    let quickmark_vector: Vec<_> = lines.map(|line| {
        let idx = line.find(' ').unwrap();
        return Quickmark {
            shortcut: line[0..idx].to_string(),
            url: line[idx+1..].to_string(),
        }
    }).collect();
    let response = serde_json::to_string(&quickmark_vector).map_err(ServerError::SerializeError)?;
    Ok(
        HttpResponse::Ok()
          .content_type("application/json")
          .body(response)
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    HttpServer::new(|| {
        // let logger = Logger::default();
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            // .wrap(logger)
            .service(history)
            .service(bookmarks)
            .service(quickmarks)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
