use std::path::{Path, PathBuf};
use pathdiff::diff_paths;
use crate::config::Config;
use crate::config::templating::GlobalTemplatingConfig;
use crate::templating::config_def::TemplateConfig;

pub mod util;
mod config;
mod languages;
mod templating;

fn main() {
    let test: TemplateConfig = toml::from_str("\
        exclusions=[\"src/main.rs\",\"config.rs\"]\
    ").unwrap();

    let global_cfg = toml::from_str("").unwrap();

    println!("{}", test.preprocess(Path::new("./"), &global_cfg)
        .unwrap().paths_included
        .into_iter()
        .map(|x| x.to_str().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("\n")
    );
}
