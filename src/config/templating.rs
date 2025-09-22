use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateConfig {
    #[serde(default = "String::new")]
    pub label: String,

    #[serde(default = "Vec::new")]
    pub exclusions: Vec<PathBuf>,
    #[serde(default = "Vec::new")]
    pub inclusions: Vec<PathBuf>,
}