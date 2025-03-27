use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::process::Command;

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(file_name, "console.log(\"Hello, Lion!\");")
}

pub fn run(file_name: &String) -> Result<(), LionError> {
    let args = vec![format!("{file_name}")];

    Command::new("node")
        .args(&args)
        .status()
        .map_err(|err| command_error("node", args, None, err))?;

    Ok(())
}

pub fn dep(dep: &String) -> Result<(), LionError> {
    let args = vec!["install".to_string(), dep.clone()];

    Command::new("npm")
        .args(&args)
        .status()
        .map_err(|err| command_error("npm", args, None, err))?;

    Ok(())
}

// Rest of the code with proj() should also return Result<(), LionError>
pub fn proj(proj_name: &String) -> Result<(), LionError> {
    let mkdir_args = vec![proj_name.clone()];

    Command::new("mkdir")
        .args(&mkdir_args)
        .status()
        .map_err(|err| command_error("mkdir", mkdir_args, None, err))?;

    let mkdir_src_args = vec![format!("{proj_name}/src")];

    Command::new("mkdir")
        .args(&mkdir_src_args)
        .status()
        .map_err(|err| command_error("mkdir", mkdir_src_args, None, err))?;

    let npm_args = vec!["init".to_string(), "-y".to_string()];

    Command::new("npm")
        .current_dir(proj_name)
        .args(&npm_args)
        .status()
        .map_err(|err| command_error("npm", npm_args, None, err))?;

    Ok(())
}
