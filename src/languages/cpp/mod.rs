use crate::utils::*;
use std::fs;
use std::process::Command;

pub fn run(file_name: &String) {
    if let Err(error) = Command::new("g++")
        .arg(file_name)
        .arg("-o")
        .arg("target/lion_compiled")
        .status()
    {
        panic!("error: {error}")
    }
    println!("\nCompiled...\n");
    if let Err(error) = Command::new("./target/lion_compiled").status() {
        panic!("error: {error}")
    }
    println!("\nRan the code successfully");
}

pub fn proj(proj_name: &String) {
    //create common directories:
    common_dir(proj_name);

    if let Err(error) = fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/external"))
    {
        panic!("error: {error}")
    }
}

pub fn new(file_name: &String) {
    writer(
        file_name,
        "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}",
    )
}
