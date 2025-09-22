pub mod rust;
pub mod templating;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
pub(crate) use crate::config::rust::RustConfig;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub rust: Option<RustConfig>,
}

impl Config {
    pub fn load(dir: impl AsRef<Path>) -> Self {
        let config_file = dir.as_ref().join(PathBuf::from("Lion.toml"));
        if config_file.exists() {
            let config = std::fs::read_to_string(config_file).unwrap();
            toml::from_str(&config).unwrap()
        } else {
            Config::default()
        }
    }
}
