use crate::utils::*;
use std::fs;
use std::process::Command;

pub fn run(file_name: &String) {
    if let Err(error) = Command::new("gcc")
        .arg(file_name)
        .arg("-o")
        .arg("target/lion_compiled")
        .status()
    {
        panic!("error: {error}")
    };
    println!("\nCompiled...\n");
    if let Err(error) = Command::new("./target/lion_compiled").status() {
        panic!("error: {error}")
    }
    println!("\nRan the code successfully");
}

pub fn project(proj_name: &String) {
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
        "#include <stdio.h>

    int main() {
        printf(\"Hello Lion!\");
        return 0;
    }",
    )
}
