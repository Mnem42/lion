use crate::utils::*;
use std::process::Command;

pub fn new(file_name: &String) {
    writer(file_name, "console.log(\"Hello, Lion!\");");
}

pub fn run(file_name: &String) {
    if let Err(error) = Command::new("node").arg(format!("{file_name}")).status() {
        panic!("error: {error}")
    }
}

pub fn dep(dep: &String) {
    Command::new("npm")
        .args(["install", dep])
        .status()
        .expect("An error occurred while trying to run npm install");
}

pub fn proj(proj_name: &String) {
    if let Err(error) = Command::new("mkdir").arg(proj_name).status() {
        panic!("error: {error}")
    }
    if let Err(error) = Command::new("mkdir")
        .arg(format!("{proj_name}/src"))
        .status()
    {
        panic!("error: {error}")
    }

    if let Err(error) = Command::new("npm")
        .current_dir(proj_name)
        .arg("init")
        .arg("-y")
        .status()
    {
        panic!("error: {error}")
    }
}
