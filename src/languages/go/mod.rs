use crate::utils::*;
use std::process::Command;

pub fn run(file_name: &String) {
    match Command::new("go")
        .arg("run")
        .arg(format!("src/{file_name}"))
        .arg("-o")
        .arg("target/lion_compiled")
        .status()
    {
        Err(e) => {
            panic!("An error occured while running your go code: {e}");
        }
        Ok(_) => {
            println!("\n\nRan the code successfully!\n");
        }
    }
}

pub fn proj(proj_name: &String) {
    if let Err(error) = Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(format!("./{proj_name}"))
        .status()
    {
        panic!("error: {error}")
    }
}

pub fn new(file_name: &String) {
    writer(
        file_name,
        "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello Lion!\")\n}",
    )
}

pub fn dep(dep: &String) {
    Command::new("go").arg("get").arg(dep);
}
