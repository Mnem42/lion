// Missing docs = missing help information
#[warn(missing_docs)]

use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use clap_derive::{Parser, Subcommand};
use crate::templating::search::search_templates;

/// Execute the provided command
pub fn exec(cli: Cli){
    match &cli.command {
        Template::IndexTemplates{extra_search_dirs} => {
            println!("search with extra dirs {:#?}", extra_search_dirs);
            let searched = &search_templates(vec![
                PathBuf::from_str(".\\").unwrap()
            ]);
            println!("searched: {:#?}", searched);
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Template,
}

#[derive(Subcommand)]
pub(crate) enum Template {
    /// Indexes templates
    ///
    /// Find templates in all the search dirs provided in the search dirs
    /// provided in the global config, with all the provided extra dirs added.
    IndexTemplates{
        extra_search_dirs: Option<Vec<PathBuf>>
    }
}