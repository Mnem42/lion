use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct TemplateConfig {
    pub label: String,

    pub exclusions: Vec<PathBuf>,
    pub inclusions: Vec<PathBuf>,
}

pub const TEMPLATE_CFG_FILENAME: &'static str = "template.toml";
