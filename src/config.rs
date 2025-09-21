use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RustConfig {
    pub run_command: Option<String>,
    pub build_command: Option<String>,
    pub test_command: Option<String>,
    pub new_workspace_command: Option<String>,
    pub new_binary_command: Option<String>,
    pub new_library_command: Option<String>,
}

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
