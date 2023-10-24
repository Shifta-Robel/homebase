use std::path::PathBuf;

pub struct Defaults;
impl Defaults{
    pub const CONFIG_PATH: &str = "./config.toml";
    pub fn bookmarks_file_path() -> PathBuf {
        PathBuf::from("/home/robel/.config/qutebrowser/bookmarks/urls")
    }
    pub fn quickmark_file_path() -> PathBuf {
        PathBuf::from("/home/robel/.config/qutebrowser/quickmarks")
    }
    pub fn quicklinks_db_path() -> PathBuf {
        PathBuf::from("./src/db/quicklinks.sqlite")
    }
    pub fn history_db_path() -> PathBuf {
        PathBuf::from("sqlite::/home/robel/.local/share/qutebrowser/history.sqlite ")
    }
    pub fn reading_list_db_path() -> PathBuf{
        PathBuf::from("./src/db/reading_list.sqlite ")
    }
}

