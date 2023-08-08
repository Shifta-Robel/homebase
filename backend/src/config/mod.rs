use std::str::FromStr;

use serde::Deserialize;

const CONFIG_FILE: &str = "./config.toml";

// #[derive(Deserialize)]
pub struct AppConfig<'a> {
    pub bookmarks_file_path: &'a str,
    pub quickmark_file_path: &'a str,
    pub history_db_path: &'a str,
    pub quicklinks_db_path: &'a str,
}

#[derive(Debug, Deserialize)]
pub enum ConfigError{
    IOError(String),
    TomlSerializeError,
}

impl<'a>  AppConfig<'a> {
    pub fn build() -> Result<AppConfig<'a>, ConfigError>{
        let contents = get_contents(CONFIG_FILE)
            .map_err(|a| ConfigError::IOError(a.to_string()))?;

        let con = serde_json::to_string(&contents).map_err(|_| ConfigError::TomlSerializeError)?;

        toml::from_str(&con)
            .map_err(|_| ConfigError::TomlSerializeError)?
        // let a: Result<AppConfig<'a>> = Ok(serde_json::from_str(&con).map_err(|_| ConfigError::TomlSerializeError)?);
        // println!("{}", con);
        // dbg!("{}", con);
        // panic!("{}",con);
        // dbg!(&con);
        // Err(ConfigError::TomlSerializeError)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for AppConfig<'a>
{
    /* ... */
    // help me implement deserialize for AppConfig
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_str(AppConfigVisitor)

    }
}
fn get_contents(path: &str) -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string(path)?)
}
