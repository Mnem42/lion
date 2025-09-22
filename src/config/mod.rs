pub mod cargo;
pub mod templating;

use std::fs::File;
use std::path::{Path, PathBuf};

pub(crate) use crate::config::cargo::CargoConfig;
use serde::{Deserialize, Serialize};

const GLOBAL_CONF_FILE: &'static Path = Path::new("global.toml");

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub rust: Option<CargoConfig>,
}

impl Config {
    pub fn create(dir: impl AsRef<Path>) {
        File::create(dir.as_ref().join(GLOBAL_CONF_FILE.to_path_buf())).unwrap();
    }

    pub fn load(dir: impl AsRef<Path>) -> Self {
        let config_file = dir.as_ref().join(GLOBAL_CONF_FILE.to_path_buf());
        if config_file.exists() {
            let config = std::fs::read_to_string(config_file).unwrap();
            toml::from_str(&config).unwrap()
        } else {
            Config::default()
        }
    }
}
