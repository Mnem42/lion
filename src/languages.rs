use std::fs;
use std::process::Command;

fn writer(file_name: &String, file_contents: &str) {
    if let Err(error) = fs::write(file_name, file_contents) {
        panic!("An error occured:\n{error}")
    }
}

pub enum MyCommand {
    Empty,
    Help,
    New,
    Dep,
    Run,
    Proj,
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
    fn project(file_ext: FileType, proj_name: &String, code_file: String);
}

impl Functions for Language {
    fn new(file_name: &String, file_ext: FileType, dep: String) {
        match file_ext {
            FileType::Py => writer(file_name, "print(\"Hello Lion!\")"),

            FileType::Rs => writer(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}"),

            FileType::Cpp => writer(
                file_name,
                "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}",
            ),

            FileType::C => writer(file_name, "#include <stdio.h>

            int main() {
                printf(\"Hello Lion!\");
                return 0;
            }"),

            FileType::Go => writer(
                file_name,
                "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello Lion!\")\n}"),

            FileType::Java => writer(file_name, "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, Lion!\");\n    }\n}"),

            FileType::Placeholder => panic!("An error occured; Unsupported file type")
        }
        if !dep.is_empty() {
            Self::dependency(file_ext, file_name, dep);
        }
    }

    fn dependency(extension: FileType, file_name: &String, dep: String) {
        let _current_dir: String = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        match extension {
            FileType::Py => {
                let new_git_url = String::from("git+") + dep.as_str();

                Command::new("pip")
                    .arg("install")
                    .arg(new_git_url)
                    .status()
                    .expect("Unable to add dependency");
                let mut dep_loop = dep;
                let dep_git = loop {
                    match dep_loop.split('/').next() {
                        None => {
                            break dep_loop;
                        }
                        Some(result) => dep_loop = result.to_string(),
                    };
                };
                //remove .git from the end:
                let (dep_name, _) = dep_git.rsplit_once('.').unwrap();

                let contents = match fs::read_to_string(file_name) {
                    Ok(value) => value,
                    Err(_) => "\nprint(\"Hello Lion!\")".to_string(),
                };

                writer(file_name, format!("import {dep_name}\n{contents}").as_str());
            }
            FileType::Rs => {
                match fs::read_to_string("Cargo.toml") {
                    Ok(value) => {
                        let (prefix, suffix) = value
                            .split_once("[dependencies]")
                            .expect("No [dependencies] section in your Cargo.toml file");

                        let final_content =
                            format!("{}[dependencies]\n{} = \"*\"{}\n", prefix, dep, suffix);
                        writer(&String::from("Cargo.toml"), final_content.as_str());
                    }
                    Err(_) => {
                        writer(
                            &String::from("Cargo.toml"),
                            format!("[dependencies]\n{dep} = \"*\"").as_str(),
                        );
                    }
                };

                writer(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}");
            }
            FileType::Cpp => {
                /*
                Command::new("git")
                    .arg("clone")
                    .arg(dep.clone())
                    .arg(format!("{current_dir}/external"))
                    .status()
                    .expect("Unable to clone git repository");
                let contents = match fs::read_to_string(file_name){
                    Ok(value) => value,
                    _ => String::from("#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}")
                };
                let final_content = format!("#include \"external/{dep}.h\"\n{contents}");
                writer(file_name, final_content.as_str());
                */
                todo!()
            }
            FileType::C => {
                todo!()
            }
            FileType::Go => {
                Command::new("go").arg("get").arg(dep);
            }
            _ => {
                eprintln!("Format not supported for external dependencies");
            }
        }
    }

    fn run(file_ext: FileType, file_name: &String) {
        if fs::DirBuilder::new()
            .recursive(true)
            .create("target")
            .is_err()
        {}
        match file_ext {
            FileType::Go => {
                match Command::new("go")
                    .arg("build")
                    .arg(format!("src/{file_name}"))
                    .arg("-o")
                    .arg("target/lion_compiled")
                    .status()
                {
                    Err(e) => {
                        panic!("An error occured while compiling your go code: {e}");
                    }
                    Ok(_) => {
                        println!("\n\nCompiled the code successfully!\n");
                    }
                }

                match Command::new("./target/lion_compiled").status() {
                    Ok(_) => {
                        println!("\n\nRan the code successfully!\n");
                    }
                    Err(e) => {
                        panic!("An error occured while running your go code: {e}");
                    }
                }
                println!("\nRan the code");
            }
            FileType::Java => {
                match Command::new("javac")
                    .arg("-d")
                    .arg("target")
                    .arg(format!("src/{file_name}"))
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
            FileType::Cpp => {
                Command::new("g++")
                    .arg(file_name)
                    .arg("-o")
                    .arg("target/lion_compiled")
                    .status()
                    .expect("An error occured; Please try again.");
                println!("\nCompiled...\n");
                Command::new("./target/lion_compiled")
                    .status()
                    .expect("An error occured; Please try again.");
                println!("\nRan the code successfully");
            }
            FileType::C => {
                Command::new("gcc")
                    .arg(file_name)
                    .arg("-o")
                    .arg("target/lion_compiled")
                    .status()
                    .expect("An error occured; Please try again.");
                println!("\nCompiled...\n");
                Command::new("./target/lion_compiled")
                    .status()
                    .expect("An error occured; Please try again.");
                println!("\nRan the code successfully");
            }
            FileType::Rs => {
                Command::new("cargo")
                    .arg("run")
                    .status()
                    .expect("An error occured; Please try again.");
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

    fn project(file_ext: FileType, proj_name: &String, code_file: String) {
        match file_ext {
            FileType::Rs => {
                Command::new("cargo")
                    .arg("new")
                    .arg(proj_name)
                    .status()
                    .expect("Error creating new rust project");
                return;
            }
            FileType::Placeholder => {
                panic!("error: Error, unknown file extension");
            }
            FileType::Cpp => {
                //create common directories:
                common_dir(proj_name);

                fs::DirBuilder::new()
                    .recursive(true)
                    .create(format!("{proj_name}/external"))
                    .expect("Error while creating externals file");
            }
            FileType::Go => {
                Command::new("go")
                    .arg("mod")
                    .arg("init")
                    .arg(format!("./{proj_name}"))
                    .status()
                    .expect("Error initialising go project");
            }
            _ => {
                //create common directories:
                common_dir(proj_name);
            }
        }

        //create code file:
        Self::new(
            &format!("{proj_name}/src/{code_file}"),
            file_ext,
            String::from(""),
        );
    }
}

fn common_dir(proj_name: &String) {
    //create project folder
    fs::DirBuilder::new()
        .recursive(true)
        .create(proj_name)
        .expect("Error creating directory");

    //Create src folder inside project directory
    fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/src"))
        .expect("Error creating directory");

    //create target folder:
    fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/target"))
        .expect("Error creating directory");
    writer(&format!("{proj_name}/.gitignore"), "/target");
}
