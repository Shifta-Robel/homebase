use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use actix_cors::Cors;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, env, fs};
// use serde_json::json;

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
enum ApiError {
    DbError(rusqlite::Error),
    SerializeError(serde_json::Error),
    IOError(std::io::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiError {}

fn error_response(e: ApiError) -> HttpResponse {
    match e {
        ApiError::DbError(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        ApiError::SerializeError(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        ApiError::IOError(e) => HttpResponse::InternalServerError().body(format!("{:?}", e))
    }
}

#[get("/get_history")]
async fn history() -> impl Responder {
    let conn = Connection::open("/home/robel/.local/share/qutebrowser/history.sqlite").map_err(ApiError::DbError);
    if let Err(e) = conn {
        return error_response(e);
    }
    let conn = conn.unwrap();
    let stmt = conn.prepare("SELECT url, title, atime FROM History")
        .map_err(ApiError::DbError);
    if let Err(e) = stmt {
        return error_response(e);
    }
    let mut stmt = stmt.unwrap();
    let history = stmt.query_map([], |row|
        Ok(
            HistoryItem {
                url: row.get(0)?,
                title: row.get(1)?,
                time: row.get(2)?
            }
        )
    ).map_err(ApiError::DbError);
    if let Err(e) = history {
        return error_response(e);
    }
    let history: Vec<_> = history.unwrap().map(|item| item.unwrap()).collect();
    let response = serde_json::to_string(&history).map_err(ApiError::SerializeError);
    if let Err(e) = response {
        return error_response(e);
    }
    HttpResponse::Ok()
      .content_type("application/json")
      .body(response.unwrap())
}

#[get("/bookmarks")]
async fn bookmarks() -> impl Responder {
    let contents = fs::read_to_string("/home/robel/.config/qutebrowser/bookmarks/urls").map_err(ApiError::IOError);
    if let Err(e) = contents {
        return error_response(e);
    }
    let contents = contents.unwrap();
    let lines = contents.lines();
    let bookmark_vector: Vec<_> = lines.map(|line| {
        let idx = line.find(' ').unwrap();
        return Bookmark {
            url: line[0..idx].to_string(),
            title: line[idx+1..].to_string(),
        }
    }).collect();
    let response = serde_json::to_string(&bookmark_vector).map_err(ApiError::SerializeError);
    if let Err(e) = response {
        return error_response(e);
    }
    HttpResponse::Ok()
      .content_type("application/json")
      .body(response.unwrap())
}

#[get("/quickmarks")]
async fn quickmarks() -> impl Responder {
    let contents = fs::read_to_string("/home/robel/.config/qutebrowser/quickmarks").map_err(ApiError::IOError);
    if let Err(e) = contents {
        return error_response(e);
    }
    let contents = contents.unwrap();
    let lines = contents.lines();
    let quickmark_vector: Vec<_> = lines.map(|line| {
        let idx = line.find(' ').unwrap();
        return Quickmark {
            shortcut: line[0..idx].to_string(),
            url: line[idx+1..].to_string(),
        }
    }).collect();
    let response = serde_json::to_string(&quickmark_vector).map_err(ApiError::SerializeError);
    if let Err(e) = response {
        return error_response(e);
    }
    HttpResponse::Ok()
      .content_type("application/json")
      .body(response.unwrap())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            .service(history)
            .service(bookmarks)
            .service(quickmarks)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
