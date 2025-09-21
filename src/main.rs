use std::path::{Path, PathBuf};
use pathdiff::diff_paths;
use crate::config::Config;
use crate::templating::config_def::TemplateConfig;

mod config;
mod languages;
mod templating;

fn main() {
    let test: TemplateConfig = toml::from_str("\
        exclusions=[\"src/main.rs\",\"config.rs\"]\
    ").unwrap();

    println!("{}", test.preprocess("src".into())
        .unwrap().paths_included
        .into_iter()
        .map(|x| x.to_str().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("\n")
    );
}
