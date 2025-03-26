use crate::utils::*;
use std::process::Command;

pub fn run(file_name: &String) {
    if let Err(error) = Command::new("cargo").arg("run").arg(file_name).status() {
        panic!("error: {error}")
    }
}

pub fn proj(proj_name: &String) {
    if let Err(error) = Command::new("cargo").arg("new").arg(proj_name).status() {
        panic!("error: {error}")
    }
}

pub fn dep(dep: &String) {
    Command::new("cargo")
        .args(["add", dep])
        .status()
        .expect("An error occurred while adding the dependency");
}

pub fn new(file_name: &String) {
    writer(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}")
}
