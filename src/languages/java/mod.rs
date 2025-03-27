use crate::utils::*;
use std::process::Command;

pub fn run(file_name: &String) {
    match Command::new("javac")
        .arg("-d")
        .arg("target")
        .arg(file_name)
        .status()
    {
        Ok(_) => println!("\nCompiled...\n"),
        Err(error) => panic!("An Error occured while compiling the Java code: {error}"),
    }
    let file_prefix = file_name
        .split('.')
        .next()
        .expect("An error occured, please check your file name");
    match Command::new("java")
        .arg("-cp")
        .arg("target")
        .arg(file_prefix)
        .status()
    {
        Ok(_) => {
            println!("\n\nRan your code successfully!")
        }
        Err(error) => {
            panic!("An error occured while running your code: {error}");
        }
    }
}

pub fn proj(proj_name: &String) {
    common_dir(proj_name);
}

pub fn new(file_name: &String) {
    writer(
        file_name,
        "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, Lion!\");\n    }\n}",
    );
}
