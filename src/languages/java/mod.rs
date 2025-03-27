use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::process::Command;

pub fn run(file_name: &String) -> Result<(), LionError> {
    let compile_args = vec!["-d".to_string(), "target".to_string(), file_name.clone()];

    Command::new("javac")
        .args(&compile_args)
        .status()
        .map_err(|err| command_error("javac", compile_args, None, err))?;

    println!("\nCompiled...\n");

    let file_prefix = file_name
        .split('.')
        .next()
        .ok_or_else(|| LionError::Custom("Invalid file name format".to_string()))?;

    let run_args = vec![
        "-cp".to_string(),
        "target".to_string(),
        file_prefix.to_string(),
    ];

    Command::new("java")
        .args(&run_args)
        .status()
        .map_err(|err| command_error("java", run_args, None, err))?;

    println!("\n\nRan your code successfully!");
    Ok(())
}

pub fn proj(proj_name: &String) -> Result<(), LionError> {
    Ok(if let Err(err) = common_dir(proj_name) {
        eprintln!(
            "An error occured while trying to create java project: {}",
            err
        );
    })
}

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(
        file_name,
        "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, Lion!\");\n    }\n}",
    )
}
