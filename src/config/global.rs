use std::path::PathBuf;
use serde::{Deserialize, Serialize};

fn default_exclusions() -> Vec<PathBuf> {
    vec![
        ".git".into(),
        ".github".into(),
        ".idea".into(),
        "Cargo.lock".into(),
    ]
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GlobalTemplatingConfig {
    #[serde(default = "default_exclusions")]
    pub default_exclusions: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default)]
    templating: GlobalTemplatingConfig,
}