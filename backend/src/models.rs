use serde::Serialize;

#[derive(Serialize)]
pub struct HistoryItem {
    pub url: String,
    pub title: String,
    pub time: i64
}

#[derive(Serialize)]
pub struct Bookmark {
    pub url: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct Quickmark {
    pub shortcut: String,
    pub url: String,
}

