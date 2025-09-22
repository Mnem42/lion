pub mod cargo;
pub mod global;
pub mod templating;

pub(crate) use crate::config::cargo::CargoConfig;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn workspace_conf_file() -> PathBuf {
    PathBuf::from_str("lion.toml").unwrap()
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub rust: Option<CargoConfig>,
}

impl Config {
    pub fn create(dir: impl AsRef<Path>) {
        File::create(dir.as_ref().join(workspace_conf_file())).unwrap();
    }

    pub fn load(dir: impl AsRef<Path>) -> Self {
        let config_file = dir.as_ref().join(workspace_conf_file());
        if config_file.exists() {
            let config = std::fs::read_to_string(config_file).unwrap();
            toml::from_str(&config).unwrap()
        } else {
            Config::default()
        }
    }
}
