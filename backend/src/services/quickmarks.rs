use rusqlite::Result;
use std::fs;
use crate::errors::ServerError;
use crate::models::Quickmark;

pub fn get_quickmarks() -> Result<Vec<Quickmark>, ServerError> {
    let contents = fs::read_to_string("/home/robel/.config/qutebrowser/quickmarks")
        .map_err(ServerError::IOError)?;
    let lines = contents.lines();
    let quickmark_vector: Vec<_> = lines.map(|line| {
        let idx = line.find(' ').unwrap();
        return Quickmark {
            shortcut: line[0..idx].to_string(),
            url: line[idx+1..].to_string(),
        }
    }).collect();
    Ok(quickmark_vector)
}
