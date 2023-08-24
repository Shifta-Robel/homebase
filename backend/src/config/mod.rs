use serde::Deserialize;

const CONFIG_FILE: &str = "./config.toml";

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub bookmarks_file_path: Option<String>,
    pub quickmark_file_path: Option<String>,
    pub history_db_path: Option<String>,
    pub quicklinks_db_path: Option<String>,
    pub reading_list_db_path: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ConfigError{
    IOError(String),
    TomlSerializeError(String),
}

impl  AppConfig {
    pub fn build() -> Result<AppConfig, ConfigError>{
        let contents = get_contents(CONFIG_FILE)
            .map_err(|a| ConfigError::IOError(a.to_string()))?;
        // let d: Result<AppConfig, toml::de::Error> = toml::from_str(&contents);
        // match d {
        //     Ok(c) => {
        //         println!("valid");
        //         Ok(c)
        //     },
        //     Err(e) => {
        //         println!("{}", e);
        //         Err(ConfigError::TomlSerializeError(e.to_string()))
        //     }
        // }
        toml::from_str(&contents)
            .map_err(|e| ConfigError::TomlSerializeError(e.to_string()))
    }
}

fn get_contents(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
