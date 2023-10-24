use std::path::PathBuf;

use rusqlite::{Connection, Result};
use crate::errors::ServerError;
use crate::models::history_item::HistoryItem;

pub fn get_history(path: PathBuf) -> Result<Vec<HistoryItem>, ServerError>{
    let conn = Connection::open(path).map_err(ServerError::DbError)?;
    // let conn = Connection::open("/home/robel/.local/share/qutebrowser/history.sqlite ").map_err(ServerError::DbError)?;
    // println!("connection: {:?}",conn);
    // println!("stmt {:?}", conn.prepare("SELECT * FROM sqlite_master where type='table' "));
    let mut stmt = conn.prepare("SELECT * FROM sqlite_master where type='table' ")
        .map_err(ServerError::DbError)?;
    // let mut stmt = conn.prepare("SELECT * FROM History")
        // .map_err(ServerError::DbError)?;
    
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
