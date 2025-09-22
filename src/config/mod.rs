pub mod rust;

use std::path::Path;

use serde::{Deserialize, Serialize};



#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub rust: Option<RustConfig>,
}

impl Config {
    pub fn load(dir: impl AsRef<Path>) -> Self {
        let config_file = dir.as_ref().join("Lion.toml");
        if config_file.exists() {
            let config = std::fs::read_to_string(config_file).unwrap();
            toml::from_str(&config).unwrap()
        } else {
            Config::default()
        }
    }
}
