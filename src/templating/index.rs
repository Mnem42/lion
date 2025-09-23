use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TemplateIndexItem {
    name: String,
    dir: PathBuf,
}


/// The index used for templates
///
/// This is where the tool gets information about various templates. Searching for
/// templates is a relatively slow thing that takes a lot of checking with the
/// file system and is also just inconvenient, so the user runs `lion index-templates`
/// to update the index from all set locations.
///
/// This index is then used by anything else that needs the location of templates,
/// and they can also get more detailed information by looking up the data in the
/// config file.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TemplateIndex {
    items: Vec<TemplateIndexItem>,
}

impl TemplateIndex {
    /// Save the index to a file
    pub fn save_index(&self, path: impl AsRef<Path>) -> Result<()> {
        let index_file = File::create(path)?;
        serde_cbor::to_writer(index_file, &self)?;

        Ok(())
    }

    /// Load the index from a file
    pub fn load_index(index: TemplateIndex, path: impl AsRef<Path>) -> Result<Self> {
        let index_file = File::open(path)?;
        Ok(serde_cbor::from_reader(index_file)?)
    }
}
