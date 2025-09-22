use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn default_run_cmd() -> String {
    "cargo run".to_owned()
}
fn default_lib_cmd() -> String {
    "cargo init --lib".to_owned()
}
fn default_bin_cmd() -> String {
    "cargo init --bin".to_owned()
}
fn default_test_cmd() -> String {
    "cargo test".to_owned()
}
fn default_build_cmd() -> String {
    "cargo build".to_owned()
}

/// A configuration for rust in a specific workspace
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoConfig {
    #[serde(default = "default_run_cmd")]
    pub run_command: String,
    #[serde(default = "default_build_cmd")]
    pub build_command: String,
    #[serde(default = "default_test_cmd")]
    pub test_command: String,
    #[serde(default = "default_bin_cmd")]
    pub new_binary_command: String,
    #[serde(default = "default_lib_cmd")]
    pub new_library_command: String,

    // Complicated
    pub new_workspace_command: Option<String>,
    // Needs runtime templating (thing for later)
    pub add_dependency_command: Option<String>,
}
