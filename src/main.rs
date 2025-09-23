use crate::config::templating::TemplateConfig;
use crate::templating::search::search_templates;
use crate::util::load_toml;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Result;

mod config;
mod languages;
mod templating;
pub mod util;

#[cfg(test)]
mod tests;

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
        &search_templates(vec![
            PathBuf::from_str(".\\").unwrap()
        ])
            .unwrap()
            .into_iter()
            .map(|x| {
                let x = x.canonicalize()?;
                println!("{:?} {}", x, x.exists());
                Ok(load_toml::<TemplateConfig>(x.clone())
                    .unwrap()
                    .resolve(x.parent().unwrap(), &global_cfg)
                )
            })
            .collect::<Vec<Result<_>>>()
    );
}
