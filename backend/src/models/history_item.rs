use serde::Serialize;

const DEFAULT_HISTORY_DB_PATH: &str = "/home/robel/.local/share/qutebrowser/history.sqlite";

#[derive(Serialize)]
pub struct HistoryItem {
    pub url: String,
    pub title: String,
    pub time: i64
}

impl HistoryItem {
    pub fn get_default_path() -> String {
        DEFAULT_HISTORY_DB_PATH.to_string() 
    }
}
