use serde::Deserialize;
use std::path::PathBuf;
use crate::defaults::Defaults;

#[derive(Deserialize, Debug, Clone)]
pub struct FileSettings {
    #[serde(default="Defaults::bookmarks_file_path")]
    pub bookmarks_file_path: PathBuf,

    #[serde(default="Defaults::quickmark_file_path")]
    pub quickmark_file_path: PathBuf,

    #[serde(default="Defaults::quicklinks_db_path")]
    pub quicklinks_db_path: PathBuf,

    #[serde(default="Defaults::history_db_path")]
    pub history_db_path: PathBuf,

    #[serde(default="Defaults::reading_list_db_path")]
    pub reading_list_db_path: PathBuf,
}

impl Default for FileSettings{
    fn default() -> Self {
        FileSettings {
            bookmarks_file_path: Defaults::bookmarks_file_path(),
            quickmark_file_path: Defaults::quickmark_file_path(),
            quicklinks_db_path: Defaults::quicklinks_db_path(),
            history_db_path: Defaults::history_db_path(),
            reading_list_db_path: Defaults::reading_list_db_path(),
        }
    }
}

