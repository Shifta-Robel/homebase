use rusqlite::Result;
use std::fs;
use crate::errors::ServerError;
use crate::models::quickmark::Quickmark;

pub fn get_quickmarks(path: &str) -> Result<Vec<Quickmark>, ServerError> {
    // let path = config.quickmark_file_path.clone().unwrap_or(Quickmark::get_default_path());
    let contents = fs::read_to_string(path)
        .map_err(ServerError::IOError)?;
    let lines = contents.lines();
    let quickmark_vector: Vec<_> = lines.map(|line| {
        let idx = line.find(' ').unwrap();
        Quickmark {
            shortcut: line[0..idx].to_string(),
            url: line[idx+1..].to_string(),
        }
    }).collect();
    Ok(quickmark_vector)
}
