use std::fs;
use std::process::Command;

pub enum MyCommand {
    Empty,
    Help,
    New,
    Dep,
    Run,
}

pub enum FileType {
    Placeholder,
    Cpp,
    Rs,
    C,
    Java,
    Go,
    Py,
}

#[allow(dead_code)]
pub struct Language {
    pub file_extension: FileType,
    pub dependency_file: String,
    pub command: MyCommand,
}

pub trait Functions {
    fn new(file_name: &String, file_ext: FileType, dependency: String);
    fn dependency(extension: FileType, file_name: &String, dep: String);
    fn run(file_ext: FileType, file_name: &String);
}

impl Functions for Language {
    fn new(file_name: &String, file_ext: FileType, dep: String) {
        match file_ext {
            FileType::Py => fs::write(file_name, "print(\"Hello Lion!\")").expect("An Unexpected error occured; please try again!"),

            FileType::Rs => fs::write(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}").expect("An Unexpected error occured; please try again!"),
            FileType::Cpp => fs::write(
                file_name,
                "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}",
            ).expect("An Unexpected error occured; please try again!"),
            FileType::C => fs::write(file_name, "#include <stdio.h>

            int main() {
                printf(\"Hello Lion!\");
                return 0;
            }").expect("An Unexpected error occured; please try again!"),
            FileType::Go => fs::write(
                file_name,
                "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello Lion!\")\n}").expect("An Unexpected error occured; please try again!"),
            FileType::Java => fs::write(file_name, "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, Lion!\");\n    }\n}").expect("An Unexpected error occured; please try again!"),
            FileType::Placeholder => panic!("An error occured; Please try again")
        }
        if !dep.is_empty() {
            Self::dependency(file_ext, file_name, dep);
        }
    }

    fn dependency(extension: FileType, file_name: &String, dep: String) {
        match extension {
            FileType::Py => {
                let contents = match fs::read_to_string(file_name) {
                    Ok(value) => value,
                    Err(_) => "\nprint(\"Hello Lion!\")".to_string(),
                };
                fs::write(file_name, format!("import {dep}\n{contents}").as_str())
                    .expect("An Unexpected error occured; please try again!")
            }
            FileType::Rs => {
                match fs::read_to_string("Cargo.toml") {
                    Ok(value) => {
                        let (before, after) = value
                            .split_once("[dependencies]")
                            .expect("You have reached unreachable code");

                        let final_content =
                            format!("{}[dependencies]\n{} = \"*\"{}\n", before, dep, after);
                        fs::write(&String::from("Cargo.toml"), final_content.as_str())
                            .expect("An Unexpected error occured; please try again!");
                    }
                    Err(_) => {
                        fs::write(
                            &String::from("Cargo.toml"),
                            format!("[dependencies]\n{dep} = \"*\""),
                        )
                        .expect("An Unexpected error occured; please try again!");
                    }
                };

                fs::write(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}")
                    .expect("An Unexpected error occured; please try again!");
            }
            FileType::Cpp => {
                let contents = match fs::read_to_string(file_name){
                    Ok(value) => value,
                    _ => String::from("#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}")
                };
                let final_content = format!("#include \"{dep}/{dep}.h\"\n{contents}");
                fs::write(file_name, final_content.as_str())
                    .expect("An Unexpected error occured; please try again!");
            }

            _ => {
                eprintln!("Format not supported for external dependencies");
            }
        }
    }

    fn run(file_ext: FileType, file_name: &String) {
        match file_ext {
            FileType::Cpp => {
                Command::new("g++")
                    .arg(file_name)
                    .arg("-o")
                    .arg("lion_compiled")
                    .status()
                    .expect("An error occured; Please try again.");
                println!("\nCompiled...\n");
                Command::new("./lion_compiled".to_string())
                    .status()
                    .expect("An error occured; Please try again.");
                println!("\nRan the code successfully");
            }
            FileType::Rs => {
                if cfg!(target_os = "windows") {
                    Command::new("rustc")
                        .arg(file_name)
                        .args(["-o", "lion_compiled"])
                        .status()
                        .expect("An error occured; Please try again.");
                    println!("\nCompiled...\n");
                    Command::new(".\\lion_compiled.exe".to_string())
                        .status()
                        .expect("An error occured; Please try again.");
                    println!("\nRan the code successfully");
                } else {
                    Command::new("rustc")
                        .arg(file_name)
                        .args(["-o", "lion_compiled"])
                        .status()
                        .expect("An error occured; Please try again.");
                    println!("\nCompiled...\n");
                    Command::new("./lion_compiled".to_string())
                        .status()
                        .expect("An error occured; Please try again.");
                    println!("\nRan the code successfully");
                };
            }
            FileType::Py => {
                Command::new("python3")
                    .arg(file_name)
                    .status()
                    .expect("An error occured, please try again");
                println!("\nRan the code successfully");
            }
            _ => {
                panic!("Running hasn't been supported yet for the specified file type");
            }
        }
    }
}
