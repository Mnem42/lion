use std::path::Path;
use crate::config::templating::TemplateConfig;

mod config;
mod languages;
mod templating;
pub mod util;

fn main() {
    let test: TemplateConfig = toml::from_str(
        "\
        exclusions=[\"src/main.rs\",\"config.rs\"]\
    ",
    )
    .unwrap();

    let global_cfg = toml::from_str("").unwrap();

    println!(
        "{}",
        test.preprocess(Path::new("./"), &global_cfg)
            .unwrap()
            .paths_included
            .into_iter()
            .map(|x| x.to_str().unwrap().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
