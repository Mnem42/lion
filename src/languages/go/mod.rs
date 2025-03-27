use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::process::Command;

pub fn run(file_name: &String) -> Result<(), LionError> {
    let args = vec![
        "run".to_string(),
        format!("src/{file_name}"),
        "-o".to_string(),
        "target/lion_compiled".to_string(),
    ];

    Command::new("go")
        .args(&args)
        .status()
        .map_err(|err| command_error("go", args, None, err))?;

    println!("\n\nRan the code successfully!\n");
    Ok(())
}

pub fn dep(dep: &String) -> Result<(), LionError> {
    let args = vec!["get".to_string(), dep.clone()];

    Command::new("go")
        .args(&args)
        .status()
        .map_err(|err| command_error("go", args, None, err))?;

    Ok(())
}

pub fn proj(proj_name: &String) -> Result<(), LionError> {
    if let Err(error) = Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(format!("./{proj_name}"))
        .status()
    {
        eprintln!("An error occurred while trying to create go project: {error}");
    };
    Ok(
        (),
    )
}

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(
        file_name,
        "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello Lion!\")\n}",
    )
}
