use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub process_names: Vec<String>,
    pub log_path: String
}

pub fn load_settings_file(path: &str) -> Settings {
    let toml_str = fs::read_to_string(path)
        .expect("Failed to read settings.toml file from the root folder");
    toml::from_str(&toml_str)
        .expect("Failed to parse settings.toml file")
}

pub struct ProcessInfo {
    pub pid: u32,
    pub start_time: u64,
    pub name: String,
    pub cmd: String
}