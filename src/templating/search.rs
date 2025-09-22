use crate::config::templating::TEMPLATE_CFG_FILENAME;
use anyhow::Result;
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn search_templates(search_locations: Vec<PathBuf>) -> Vec<PathBuf> {
    let tmp: Result<Vec<_>> = search_locations
        .into_iter()
        .map(|x| WalkDir::new(x).into_iter())
        .flatten()
        .map(|x| {
            let path = x?.clone().path().join(PathBuf::from(TEMPLATE_CFG_FILENAME));
            if path.exists() {
                return Ok(path);
            }
            unreachable!()
        })
        .collect();

    return vec![];
}
