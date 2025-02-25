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
    Ts,
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
            FileType::Ts =>{
                writer(file_name, "console.log(\"Hello, Lion!\";");
            },
            FileType::Placeholder => panic!("An error occured; Unsupported file type"),

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
                if dep.contains(".git") {
                    let new_git_url = String::from("git+") + dep.as_str();

                    match Command::new("pip").arg("install").arg(new_git_url).status() {
                        Err(error) => panic!("error: {error}"),
                        _ => {}
                    }
                }

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
                match Command::new("g++")
                    .arg(file_name)
                    .arg("-o")
                    .arg("target/lion_compiled")
                    .status()
                {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
                println!("\nCompiled...\n");
                match Command::new("./target/lion_compiled").status() {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
                println!("\nRan the code successfully");
            }
            FileType::C => {
                match Command::new("gcc")
                    .arg(file_name)
                    .arg("-o")
                    .arg("target/lion_compiled")
                    .status()
                {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                };
                println!("\nCompiled...\n");
                match Command::new("./target/lion_compiled").status() {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
                println!("\nRan the code successfully");
            }
            FileType::Rs => match Command::new("cargo").arg("run").status() {
                Err(error) => panic!("error: {error}"),
                _ => {}
            },
            FileType::Py => {
                match Command::new("python3").arg(file_name).status() {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
                println!("\nRan the code successfully");
            }
            FileType::Placeholder => {
                panic!("Running hasn't been supported yet for the specified file type");
            }
            FileType::Ts => {
                match Command::new("tsc").status() {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
                let (name, _) = file_name.split_once(".").unwrap();
                match Command::new("node").arg(format!("src/{name}.js")).status() {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
            }
        }
    }

    fn project(file_ext: FileType, proj_name: &String, code_file: String) {
        match file_ext {
            FileType::Rs => {
                match Command::new("cargo").arg("new").arg(proj_name).status() {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
                return;
            }

            FileType::Cpp => {
                //create common directories:
                common_dir(proj_name);

                match fs::DirBuilder::new()
                    .recursive(true)
                    .create(format!("{proj_name}/external"))
                {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
            }
            FileType::Go => {
                match Command::new("go")
                    .arg("mod")
                    .arg("init")
                    .arg(format!("./{proj_name}"))
                    .status()
                {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
            }
            FileType::Ts => {
                match Command::new("mkdir").arg(proj_name).status() {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
                match Command::new("mkdir")
                    .arg(format!("{proj_name}/src"))
                    .status()
                {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }

                match Command::new("npx")
                    .current_dir(proj_name)
                    .arg("tsc")
                    .arg("--init")
                    .status()
                {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }
            }
            FileType::Placeholder => {
                panic!("error: Error, unknown or missing file extension");
            }
            FileType::Py => {
                match Command::new("python3")
                    .arg("-m")
                    .arg("venv")
                    .arg(proj_name)
                    .status()
                {
                    Err(error) => panic!("error: {error}"),
                    _ => {}
                }

                Self::new(
                    &format!("{proj_name}/{code_file}"),
                    file_ext,
                    String::from(""),
                );
                return;
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
    match fs::DirBuilder::new().recursive(true).create(proj_name) {
        Err(error) => panic!("error: {error}"),
        _ => {}
    }

    //Create src folder inside project directory
    match fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/src"))
    {
        Err(error) => panic!("error: {error}"),
        _ => {}
    }

    //create target folder:
    match fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/target"))
    {
        Err(error) => panic!("error: {error}"),
        _ => {}
    }
    writer(&format!("{proj_name}/.gitignore"), "/target");
}
