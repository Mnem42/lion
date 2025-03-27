use crate::utils::*;
use std::process::Command;

pub fn new(file_name: &String) {
    writer(file_name, "console.log(\"Hello, Lion!\");");
}

pub fn run(file_name: &String) {
    if let Err(error) = Command::new("tsc").status() {
        panic!("error: {error}")
    }
    let (name, _) = file_name.split_once(".").unwrap();
    if let Err(error) = Command::new("node").arg(format!("{name}.js")).status() {
        panic!("error: {error}")
    }
}

pub fn dep(dep: &String) {
    Command::new("npm")
        .args(["install", dep])
        .status()
        .expect("An error occurred while trying to run npm install");
    Command::new("npm")
        .args(["install", "--save-dev", format!("@types/{dep}").as_str()])
        .status()
        .expect("Error while installing typescript types");
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

    if let Err(error) = Command::new("npx")
        .current_dir(proj_name)
        .arg("tsc")
        .arg("--init")
        .status()
    {
        panic!("error: {error}")
    }
}
