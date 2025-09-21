use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use anyhow::Result;
use pathdiff::diff_paths;

const fn default_exclusions() -> Vec<PathBuf> {
    vec![]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateConfig{
    #[serde(default="default_exclusions")]
    pub exclusions: Vec<PathBuf>
}

#[derive(Debug)]
pub struct PreprocessedTemplateConfig{
    pub paths_included: Vec<PathBuf>
}

impl TemplateConfig{
    pub fn preprocess(&self, root: PathBuf) -> Result<PreprocessedTemplateConfig,anyhow::Error> {
        let normalised_excls = self.exclusions
            .iter()
            .filter_map(|x| x.canonicalize().ok())
            .collect::<Vec<_>>();

        let inclusions: Result<Vec<PathBuf>,_> = WalkDir::new(root.clone())
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
                        match PathBuf::from(root.clone())
                            .canonicalize() {
                            Ok(x) => x,
                            Err(x) => return Some(Err(x))
                        }
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