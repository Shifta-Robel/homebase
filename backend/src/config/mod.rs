use crate::defaults::Defaults;
use crate::errors::UserError;
use crate::models::background::{BackgroundType, Pattern};
use crate::models::file_settings::FileSettings;
use serde::Deserialize;
use std::path::PathBuf;

pub struct AppConfig {
    pub files: FileSettings,
    pub background: BackgroundType,
}

impl AppConfig {
    pub fn build() -> Result<Self, UserError> {
        let inter = AppConfigInter::build();
        Ok(AppConfig {
            files: inter.files.clone().unwrap(),
            background: inter.select_bg_type()?,
        })
    }
}

#[derive(Deserialize, Debug)]
struct AppConfigInter {
    #[serde(default)]
    files: Option<FileSettings>,
    bg_image: Option<BackgroundImageSettings>,
    bg_color: Option<BackgroundColorSettings>,
    bg_gradient: Option<BackgroundGradientSettings>,
    bg_pattern: Option<Pattern>,
}

impl AppConfigInter {
    fn build() -> Self {
        let contents = get_contents(Defaults::CONFIG_PATH).unwrap();
        let contents = contents.as_str();
        let conf_inter: AppConfigInter = toml::from_str(contents).unwrap();
        conf_inter
    }
    fn select_bg_type(self) -> Result<BackgroundType, UserError> {
        if let Some(bg) = self.bg_image {
            Ok(BackgroundType::Image {
                src: bg.src.ok_or(UserError::InvalidConfigurationError(
                    "Background image src missing".to_string(),
                ))?,
                blur: bg.blur.unwrap(),
                fill: bg.fill.unwrap_or("Stretch".to_string()), // replace with an enum
                pattern: self.bg_pattern,
            })
        } else if let Some(bg) = self.bg_gradient {
            Ok(BackgroundType::ColorGradient {
                from: bg.from.ok_or(UserError::InvalidConfigurationError(
                    "Color gradient :from: missing".to_string(),
                ))?,
                to: bg.to.ok_or(UserError::InvalidConfigurationError(
                    "Color gradient :to: missing".to_string(),
                ))?,
                angle: bg.angle.unwrap(),
                pattern: self.bg_pattern,
            })
        } else if let Some(bg) = self.bg_color {
            Ok(BackgroundType::Color {
                color: bg.color,
                pattern: self.bg_pattern,
            })
        } else {
            //default
            Ok(BackgroundType::Color {
                color: String::from("default"),
                pattern: None,
            })
        }
    }
}

fn get_contents(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

#[derive(Deserialize, Debug)]
struct BackgroundImageSettings {
    src: Option<PathBuf>,
    #[serde(default)]
    blur: Option<i32>,
    #[serde(default)]
    fill: Option<String>,
}

#[derive(Deserialize, Debug)]
struct BackgroundColorSettings {
    color: String,
}

#[derive(Deserialize, Debug)]
struct BackgroundGradientSettings {
    // #[serde(default="some")]
    from: Option<String>,
    // #[serde(default="some")]
    to: Option<String>,
    #[serde(default)]
    angle: Option<i32>,
}

// #[derive(Deserialize, Debug)]
// pub struct BackgroundPatternSetting{
//     pub pattern: Option<String>, // make it an enum of predefined patterns
//     pub custom: Option<String>,
//     /// color for the svg pattern
//     pub color: Option<String>,
//     // blurable
//     // gap
// }

//
// use configparser::ini::Ini;
// use serde::Deserialize;
// use toml::Table;
// use crate::models::background::BackgroundType;
//
// const CONFIG_FILE: &str = "./config.toml";
//
// #[derive(Deserialize, Clone)]
// pub struct AppConfig {
//     // pub bookmarks_file_path: Option<String>,
//     // pub quickmark_file_path: Option<String>,
//     // pub history_db_path: Option<String>,
//     // pub quicklinks_db_path: Option<String>,
//     // pub reading_list_db_path: Option<String>,
//     pub files: Option<FilePaths>,
//     pub background: BackgroundType,
// }
//
// #[derive(Deserialize, Clone)]
// struct FilePaths {
//     pub bookmarks_file_path: Option<String>,
//     pub quickmark_file_path: Option<String>,
//     pub history_db_path: Option<String>,
//     pub quicklinks_db_path: Option<String>,
//     pub reading_list_db_path: Option<String>,
// }
//
// #[derive(Debug, Deserialize, Clone)]
// pub enum ConfigError{
//     IOError(String),
//     TomlSerializeError(String),
// }
//
// impl  AppConfig {
//     pub fn build() -> Result<AppConfig, ConfigError>{
//         let contents = get_contents(CONFIG_FILE)
//             .map_err(|a| ConfigError::IOError(a.to_string()))?;
//         let d: Result<AppConfig, toml::de::Error> = toml::from_str(&contents);
//         let tb = contents.as_str().parse::<Table>().unwrap();
//
//         // match d {
//         //     Ok(c) => {
//         //         println!("valid");
//         //         Ok(c)
//         //     },
//         //     Err(e) => {
//         //         println!("{}", e);
//         //         Err(ConfigError::TomlSerializeError(e.to_string()))
//         //     }
//         // }
//         let config = Ini::new();
//         // let map = config.load(CONFIG_FILE);
//
//         // AppConfig {
//         //     bookmarks_file_path: Some(config.get("files", "bookmarks_file_path")) ,
//         //     quickmark_file_path: Some(config.get("files", "quickmark_file_path")),
//         //     history_db_path: Some(config.get("files", "history_db_path")),
//         //     quicklinks_db_path: Some(config.get("files", "quicklinks_db_path")),
//         //     reading_list_db_path: Some(config.get("files", "reading_list_db_path")),
//         //     background: Ba
//         // };
//         toml::from_str(&contents)
//             .map_err(|e| ConfigError::TomlSerializeError(e.to_string()))
//     }
// }
//
// fn get_contents(path: &str) -> Result<String, std::io::Error> {
//     std::fs::read_to_string(path)
// }
//
// fn get_bg_type(config: Ini) -> BackgroundType {
//
// }
