use serde::Serialize;

#[derive(Serialize)]
pub struct Quickmark {
    pub shortcut: String,
    pub url: String,
}
