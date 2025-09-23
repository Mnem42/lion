use crate::config::templating::TemplateConfig;
use crate::templating::search::search_templates;
use crate::util::load_toml;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Result;
use clap::Parser;
use crate::commands::exec;

mod config;
mod languages;
mod templating;
pub mod util;

#[cfg(test)]
mod tests;
mod commands;

fn main() {
    exec(Parser::parse());
}
