use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn default_exclusions() -> Vec<PathBuf> {
    vec![
        ".git".into(),
        ".github".into(),
        ".idea".into(),
        "Cargo.lock".into(),
    ]
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalTemplatingConfig {
    pub default_exclusions: Vec<PathBuf>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalConfig {
    templating: GlobalTemplatingConfig,
}
