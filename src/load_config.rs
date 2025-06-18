use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_title: String,
    pub title_width: f32,
    pub chrome: ChromePaths,
    pub logo: Logo, // ✅ Add this line
    pub button_labels: ButtonLabels,
}

#[derive(Debug, Deserialize)]
pub struct ChromePaths {
    pub local_state_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Logo {
    pub name: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize)]
pub struct ButtonLabels {
    pub button_1: String,
    pub button_2: String,
    pub button_3: String,
    pub button_4: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
