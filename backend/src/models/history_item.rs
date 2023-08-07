use serde::Serialize;

#[derive(Serialize)]
pub struct HistoryItem {
    pub url: String,
    pub title: String,
    pub time: i64
}
