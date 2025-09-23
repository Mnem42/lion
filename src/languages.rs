use std::{fs, process::Command};

use anyhow::{Result, anyhow};

use crate::config::CargoConfig;

pub trait Language {
    fn run(&self) -> Result<()>;
    fn proj(&self) -> Result<()>;
    fn new(&self) -> Result<()>;
    fn add(&self) -> Result<()>;
}

pub enum Languages {
    Cpp,
    Rust,
    Go,
    Java,
    Python,
    Typescript,
    Javascript,
    Gleam,
    Zig,
    C,
}

impl TryFrom<&str> for Languages {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "cpp" | "c++" => Ok(Languages::Cpp),
            "rust" | "rs" => Ok(Languages::Rust),
            "go" | "golang" => Ok(Languages::Go),
            "java" => Ok(Languages::Java),
            "python" | "py " => Ok(Languages::Python),
            "typescript" | "ts" => Ok(Languages::Typescript),
            "javascript" | "js" => Ok(Languages::Javascript),
            "gleam" => Ok(Languages::Gleam),
            "zig" => Ok(Languages::Zig),
            "c" => Ok(Languages::C),
            _ => Err(anyhow!("Invalid language")),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum RustProjectType {
    #[default]
    Binary,
    Library,
    Workspace(RustWorkspace),
}

#[derive(Debug, Clone)]
pub struct Rust {
    pub config: Option<CargoConfig>,
    pub proj_name: String,
    pub dir_path: String,
    pub project_type: RustProjectType,
    pub target_file: Option<String>,
    pub dep: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct RustWorkspace {
    pub target_member: Option<String>,
    pub members: Vec<String>,
}

impl Language for Rust {
    fn run(&self) -> Result<()> {
        match self.project_type.clone() {
            RustProjectType::Binary => {
                Command::new("cargo")
                    .arg("run")
                    .status()
                    .map_err(|err| anyhow!("Failed to run binary: {}", err))?;
            }
            RustProjectType::Library => {
                Command::new("cargo")
                    .arg("build")
                    .status()
                    .map_err(|err| anyhow!("Failed to build library: {}", err))?;
            }
            RustProjectType::Workspace(package) => {
                let target_member = match package.target_member {
                    Some(member) => member,
                    None => return Err(anyhow!("Target member not specified")),
                };
                Command::new("cargo")
                    .args(["run", "--package", &target_member])
                    .status()
                    .map_err(|err| anyhow!("Failed to build workspace: {}", err))?;
            }
        }
        Ok(())
    }

    fn proj(&self) -> Result<()> {
        match self.project_type.clone() {
            RustProjectType::Binary => {
                Command::new("cargo")
                    .arg("new")
                    .arg(self.proj_name.clone())
                    .status()
                    .map_err(|err| anyhow!("Failed to create binary project: {}", err))?;
            }
            RustProjectType::Library => {
                Command::new("cargo")
                    .arg("new")
                    .arg(self.proj_name.clone())
                    .status()
                    .map_err(|err| anyhow!("Failed to create library project: {}", err))?;
            }
            RustProjectType::Workspace(ws) => {
                fs::create_dir_all(&self.proj_name)?;
                let mut cargo_toml = include_str!("../samples/Cargo.toml").to_string();
                let members: Vec<String> = ws
                    .members
                    .iter()
                    .map(|member| format!("\"{}\"", member))
                    .collect();
                cargo_toml = cargo_toml.replace("{{MEMBERS}}", &members.join(","));
                fs::write(self.proj_name.clone() + "Cargo.toml", cargo_toml)?;
            }
        }
        Ok(())
    }

    fn new(&self) -> Result<()> {
        let new_file = match self.target_file.clone() {
            Some(file) => file,
            None => return Err(anyhow!("Target file not specified")),
        };

        fs::write(
            new_file.replace("~", std::env::home_dir().unwrap().to_str().unwrap()),
            include_str!("../samples/sample.rs"),
        )?;

        Ok(())
    }

    fn add(&self) -> Result<()> {
        let dep = match self.dep.clone() {
            Some(dep) => dep,
            None => return Err(anyhow!("Dependency not specified")),
        };
        match self.project_type.clone() {
            RustProjectType::Binary => {
                Command::new("cargo")
                    .arg("add")
                    .arg(dep)
                    .status()
                    .map_err(|err| anyhow!("Failed to add dependency: {}", err))?;
            }
            RustProjectType::Library => {
                Command::new("cargo")
                    .arg("add")
                    .arg(dep)
                    .status()
                    .map_err(|err| anyhow!("Failed to add dependency: {}", err))?;
            }
            RustProjectType::Workspace(ws) => {
                let target_member = match ws.target_member {
                    Some(member) => member,
                    None => return Err(anyhow!("Target member not specified")),
                };
                Command::new("cargo")
                    .arg("add")
                    .arg(dep)
                    .arg(target_member)
                    .status()
                    .map_err(|err| anyhow!("Failed to add dependency: {}", err))?;
            }
        }
        Ok(())
    }
}
