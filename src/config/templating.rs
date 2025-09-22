use std::path::PathBuf;

fn default_exclusions() -> Vec<PathBuf> {
    vec![
        ".git".into(),
        "Cargo.lock".into()
    ]
}

pub struct GlobalTemplatingConfig {
    pub default_exclusions: Vec<PathBuf>
}