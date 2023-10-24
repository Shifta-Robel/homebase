use std::path::PathBuf;

use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum BackgroundType {
    Image {
        src: PathBuf,
        blur: i32,
        fill: String, // figure this out
        pattern: Option<Pattern>
    },
    Color {
        color: String,
        pattern: Option<Pattern>,
    },
    ColorGradient {
        from: String,
        to: String,
        angle: i32,
        pattern: Option<Pattern>,
    }
}

#[derive(Serialize,Deserialize, Debug, Clone)]
pub struct Pattern{
    pattern: String, // make ths an enum
    #[serde(default)]
    custom: String,
    color: String,
    // thickness
}

// #[derive(Deserialize, Clone)]
// pub struct Background {
//     pub image: Option<String>,
//     pub color: Option<String>,
//     pub blur: Option<i32>,
//     pub pattern: (Option<String>, i32),
//     pub gradient: Option<Gradient>
// }
//
// #[derive(Deserialize, Clone)]
// pub struct Gradient{
//     from: i32,
//     to: i32,
// }
