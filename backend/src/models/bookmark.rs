use serde::Serialize;

#[derive(Serialize)]
pub struct Bookmark {
    pub url: String,
    pub title: String,
}

