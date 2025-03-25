use crate::utils::*;
use std::process::Command;

pub fn new(file_name: &String) {
    writer(file_name, "print(\"Hello Lion!\")")
}

pub fn run(file_name: &String) {
    if let Err(error) = Command::new("python3").arg(file_name).status() {
        panic!("error: {error}")
    }
    println!("\nRan the code successfully");
}

pub fn proj(proj_name: &String) {
    if let Err(error) = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(proj_name)
        .status()
    {
        panic!("error: {error}")
    }

    new(&format!("{}/main.py", proj_name));
}

pub fn dependency(dep: &String) {
    if dep.contains(".git") {
        let new_git_url = String::from("git+") + dep.as_str();

        if let Err(error) = Command::new("pip").arg("install").arg(new_git_url).status() {
            panic!("error: {error}")
        }
    } else {
        Command::new("pip")
            .arg("install")
            .arg(dep.clone())
            .status()
            .expect("Error adding dependency.\n");
    }
}
