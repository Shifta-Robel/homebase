use rusqlite::{Connection, Result};
use crate::errors::ServerError;
use crate::models::history_item::HistoryItem;

pub fn get_history(path: &str) -> Result<Vec<HistoryItem>, ServerError>{
    let conn = Connection::open(path).map_err(ServerError::DbError)?;
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
    Ok(history)
}
