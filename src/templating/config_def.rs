use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use anyhow::Result;
use pathdiff::diff_paths;
use crate::config::Config;
use crate::itry;

// TODO: make this take stuff from configs instead of being hard coded
const DEFAULT_EXCLUSIONS: [&'static str; 2] = [
    ".git",
    "node_modules",
];

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateConfig{
    #[serde(default="Vec::new")]
    pub exclusions: Vec<PathBuf>,
    #[serde(default="Vec::new")]
    pub inclusions: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct PreprocessedTemplateConfig{
    pub paths_included: Vec<PathBuf>
}

impl TemplateConfig{
    pub fn preprocess(self, root: &Path, config: &Config) -> Result<PreprocessedTemplateConfig,anyhow::Error> {
        // I *don't* like this implementation, but can't think of a saner one

        let mut exclusions = Vec::from(DEFAULT_EXCLUSIONS.map(|x| x.into()));
        exclusions.extend_from_slice(&self.exclusions);

        let normalised_excls = exclusions
            .iter()
            .filter_map(|x| x.canonicalize().ok())
            .filter(|x| !self.inclusions.contains(x))
            .collect::<Vec<_>>();

        let inclusions: Result<_> = WalkDir::new(root.to_path_buf())
            .into_iter()
            .map(|x| {
                let x = x?.path().to_path_buf();
                return Ok(x.canonicalize()?);
            })
            .zip(std::iter::repeat(normalised_excls))
            .filter_map(|(x, excls)| {
                if let Ok(x) = &x {
                    if excls.contains(x) { return None }
                    let diffed = diff_paths(
                        x.clone(),
                        itry!(PathBuf::from(root.to_path_buf()).canonicalize())
                    )?;
                    return Some(Ok(diffed))
                }
                Some(x.into())
            })
            .collect();

        Ok(PreprocessedTemplateConfig {
            paths_included: inclusions?
        })
    }
}