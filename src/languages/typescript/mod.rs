use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::process::Command;

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(file_name, "console.log(\"Hello, Lion!\");")
}

pub fn run(file_name: &String) -> Result<(), LionError> {
    Command::new("tsc")
        .status()
        .map_err(|err| command_error("tsc", vec![], None, err))?;

    let (name, _) = file_name
        .split_once(".")
        .ok_or_else(|| LionError::Custom("Invalid file name format".to_string()))?;

    let args = vec![format!("{name}.js")];

    Command::new("node")
        .args(&args)
        .status()
        .map_err(|err| command_error("node", args, None, err))?;

    Ok(())
}

pub fn dep(dep: &String) -> Result<(), LionError> {
    let npm_install_args = vec!["install".to_string(), dep.clone()];

    Command::new("npm")
        .args(&npm_install_args)
        .status()
        .map_err(|err| command_error("npm", npm_install_args.clone(), None, err))?;

    let npm_types_args = vec![
        "install".to_string(),
        "--save-dev".to_string(),
        format!("@types/{dep}"),
    ];

    Command::new("npm")
        .args(&npm_types_args)
        .status()
        .map_err(|err| command_error("npm", npm_types_args, None, err))?;

    Ok(())
}

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

    let npx_args = vec!["tsc".to_string(), "--init".to_string()];

    Command::new("npx")
        .current_dir(proj_name)
        .args(&npx_args)
        .status()
        .map_err(|err| command_error("npx", npx_args, None, err))?;

    Ok(())
}
