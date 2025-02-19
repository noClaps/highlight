use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Highlight {
    pub color: Option<String>,
    pub font_weight: Option<u16>,
    pub font_style: Option<String>,
    pub background_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineNumbers {
    pub color: String,
    pub right_space: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub line_numbers: Option<LineNumbers>,
    pub highlights: HashMap<String, Highlight>,
}
