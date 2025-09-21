use anyhow::{Result, anyhow};

pub trait Language {
    fn run(&self) -> Result<()>;
    fn proj(&self) -> Result<()>;
    fn new(&self) -> Result<()>;
}
