use crate::config::global::GlobalTemplatingConfig;
use crate::config::templating::TemplateConfig;
use anyhow::Result;
use pathdiff::diff_paths;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct TemplateInfo {
    pub paths_included: Vec<PathBuf>,
}

impl TemplateConfig {
    pub fn preprocess(self, root: &Path, config: &GlobalTemplatingConfig) -> Result<TemplateInfo> {
        // I *don't* like this implementation, but can't think of a saner one

        let mut exclusions = config.default_exclusions.clone();
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
            .filter(|x| {
                match x {
                    Ok(x) => !normalised_excls.iter().any(|excl| {
                        x.starts_with(excl.to_path_buf()) || normalised_excls.contains(x)
                    }),
                    Err(_) => true, // Keep errors
                }
            })
            .map(|x| {
                if let Ok(x) = &x {
                    return Ok(diff_paths(
                        x.clone(),
                        PathBuf::from(root.to_path_buf()).canonicalize()?,
                    )
                    .unwrap()); // None should be physically impossible in this case
                } else {
                    x
                }
            })
            .collect();

        Ok(TemplateInfo {
            paths_included: inclusions?,
        })
    }
}
