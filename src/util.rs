use anyhow::Result;
use serde::de::DeserializeOwned;
use std::path::Path;

/// Like try, but for iterators that return [`Option<Result<_, _>>`].
///
/// NOTE: ripped out of the walkdir crate
///
/// [`Option<Result<_, _>>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
// Unused for now
#[macro_export]
macro_rules! itry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(From::from(err))),
        }
    };
}

/// Helper function to load from a path *if* the file exists, and in other cases,
/// call `T::Default` to get a default value.
///
/// This is mainly to make loading configs a bit nicer and less repetitive
pub fn load_toml<T: DeserializeOwned + Default>(path: impl AsRef<Path>) -> Result<T> {
    if path.as_ref().exists() {
        let config = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&config)?)
    } else {
        Ok(T::default())
    }
}
