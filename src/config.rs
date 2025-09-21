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

