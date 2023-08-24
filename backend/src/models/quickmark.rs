use serde::Serialize;

const DEFAULT_QUICKMARK_PATH: &str = "/home/robel/.config/qutebrowser/quickmarks";
#[derive(Serialize)]
pub struct Quickmark {
    pub shortcut: String,
    pub url: String,
}

impl Quickmark {
    pub fn get_default_path() -> String {
        DEFAULT_QUICKMARK_PATH.to_string()
    }
}
