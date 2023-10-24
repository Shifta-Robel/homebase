use rusqlite::Result;
use std::fs;
use std::path::PathBuf;
use crate::errors::ServerError;
use crate::models::bookmark::Bookmark;

pub fn get_bookmarks(path: PathBuf) -> Result<Vec<Bookmark>, ServerError> {
    let contents = fs::read_to_string(path).map_err(ServerError::IOError)?;
    let lines = contents.lines();
    let bookmark_vector: Vec<_> = lines.map(|line| {
        let idx = line.find(' ').unwrap();
        Bookmark {
            url: line[0..idx].to_string(),
            title: line[idx+1..].to_string(),
        }
    }).collect();
    Ok(bookmark_vector)
}
