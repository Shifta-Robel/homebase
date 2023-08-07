use serde::Deserialize;

const CONFIG_FILE: &str = "../../config.toml";

#[derive(Deserialize)]
pub struct AppConfig {
    pub bookmarks_file_path: String,
    pub quickmark_file_path: String,
    pub history_db_path: String,
    pub quicklinks_db_path: String,
}

#[derive(Debug, Deserialize)]
pub enum ConfigError{
    IOError,
    TomlSerializeError,
}

impl AppConfig {
    pub fn build() -> Result<AppConfig, ConfigError>{
        let contents = get_contents(CONFIG_FILE)
            .map_err(|_| ConfigError::IOError)?;
        toml::from_str(&contents)
            .map_err(|_| ConfigError::TomlSerializeError)?
    }
}

fn get_contents(path: &str) -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string(path)?)
}
