use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::process::Command;

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(file_name, "print(\"Hello Lion!\")")
}

pub fn run(file_name: &String) -> Result<(), LionError> {
    let args = vec![file_name.clone()];

    Command::new("python3")
        .args(&args)
        .status()
        .map_err(|err| command_error("python3", args, None, err))?;

    println!("\nRan the code successfully");
    Ok(())
}

pub fn dependency(dep: &String) -> Result<(), LionError> {
    if dep.contains(".git") {
        let new_git_url = String::from("git+") + dep.as_str();
        let args = vec!["install".to_string(), new_git_url];

        Command::new("pip")
            .args(&args)
            .status()
            .map_err(|err| command_error("pip", args, None, err))?;
    } else {
        let args = vec!["install".to_string(), dep.clone()];

        Command::new("pip")
            .args(&args)
            .status()
            .map_err(|err| command_error("pip", args, None, err))?;
    }

    Ok(())
}

pub fn proj(proj_name: &String) -> Result<(), LionError> {
    let args = vec![
        "-m".to_string(),
        "venv".to_string(),
        format!("{}/venv", proj_name),
    ];

    if let Err(err) = common_dir(proj_name) {
        eprintln!("Failed to create common directories: {}", err);
    }

    Command::new("python3")
        .args(&args)
        .status()
        .map_err(|err| command_error("python3", args, None, err))?;

    if let Err(err) = new(&format!("{}/src/main.py", proj_name)) {
        eprintln!("Failed to create Python file: {}", err);
    }

    Ok(())
}
