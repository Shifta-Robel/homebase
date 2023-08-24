use serde::Serialize;

const BOOKMARK_FILE_PATH: &str = "/home/robel/.config/qutebrowser/bookmarks/urls";

#[derive(Serialize)]
pub struct Bookmark {
    pub url: String,
    pub title: String,
}

impl Bookmark {
    pub fn get_default_path() -> String {
       BOOKMARK_FILE_PATH.to_string() 
    }
}

