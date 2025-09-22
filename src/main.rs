use crate::config::templating::TemplateConfig;
use crate::templating::search::search_templates;
use crate::util::load_toml;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
        "{:#?}",
        search_templates(vec![PathBuf::from_str(".\\").unwrap()])
            .unwrap()
            .into_iter()
            .map(|x| {
                println!("{:?}", x);
                load_toml::<TemplateConfig>(x.as_path())
                    .unwrap()
                    .preprocess(x.parent().unwrap(), &global_cfg)
            })
            .collect::<Vec<_>>()
    );

    /*println!(
        "{}",
        test.preprocess(Path::new("./"), &global_cfg)
            .unwrap()
            .paths_included
            .into_iter()
            .map(|x| x.to_str().unwrap().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );*/
}
