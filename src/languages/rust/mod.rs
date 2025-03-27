use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::process::Command;

pub fn run(file_name: &String) -> Result<(), LionError> {
    let args = vec!["run".to_string(), file_name.clone()];

    Command::new("cargo")
        .args(&args)
        .status()
        .map_err(|err| command_error("cargo", args, None, err))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(LionError::CommandError {
                    command: "cargo".to_string(),
                    args: vec!["run".to_string(), file_name.clone()],
                    exit_status: Some(status),
                    source: std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Command exited with non-zero status",
                    ),
                })
            }
        })
}

pub fn proj(proj_name: &String) -> Result<(), LionError> {
    Ok(
        if let Err(error) = Command::new("cargo").arg("new").arg(proj_name).status() {
            eprintln!("Error while trying to create Rust project: {}", error);
        },
    )
}

pub fn dep(dep: &String) {
    Command::new("cargo")
        .args(["add", dep])
        .status()
        .expect("An error occurred while adding the dependency");
}

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}")
}
