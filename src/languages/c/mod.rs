use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::fs;
use std::process::Command;

pub fn run(file_name: &String) -> Result<(), LionError> {
    let compile_args = vec![
        file_name.clone(),
        "-o".to_string(),
        "target/lion_compiled".to_string(),
    ];

    Command::new("gcc")
        .args(&compile_args)
        .status()
        .map_err(|err| command_error("gcc", compile_args, None, err))?;

    println!("\nCompiled...\n");

    Command::new("./target/lion_compiled")
        .status()
        .map_err(|err| command_error("./target/lion_compiled", vec![], None, err))?;

    println!("\nRan the code successfully");
    Ok(())
}

pub fn proj(proj_name: &String) -> Result<(), LionError> {
    if let Err(err) = common_dir(proj_name) {
        eprintln!("Failed to create common directories: {}", err);
    }

    if let Err(error) = fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/external"))
    {
        println!("An error occurred while trying to create C project: {error}")
    };
    Ok(())
}

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(
        file_name,
        "#include <stdio.h>

    int main() {
        printf(\"Hello Lion!\");
        return 0;
    }",
    )
}
