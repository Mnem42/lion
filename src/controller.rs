use std::fs;

use crate::errors::LionError;
use crate::languages::c;
use crate::languages::cpp;
use crate::languages::go;
use crate::languages::java;
use crate::languages::javascript;
use crate::languages::python;
use crate::languages::rust;
use crate::languages::typescript;

#[derive(PartialEq)]
pub enum FileType {
    Placeholder,
    Cpp,
    Rs,
    C,
    Java,
    Go,
    Py,
    Ts,
    Js,
}

#[allow(dead_code)]
pub struct Language {
    pub file_extension: FileType,
}

impl Language {
    pub fn new(file_name: &String, file_ext: FileType, dep: String) -> Result<(), LionError> {
        match file_ext {
            FileType::Py => python::new(file_name)?,
            FileType::Rs => rust::new(file_name)?,
            FileType::Cpp => cpp::new(file_name)?,
            FileType::C => c::new(file_name)?,
            FileType::Go => go::new(file_name)?,
            FileType::Java => java::new(file_name)?,
            FileType::Ts => typescript::new(file_name)?,
            FileType::Js => javascript::new(file_name)?,
            FileType::Placeholder => {
                return Err(LionError::UnsupportedFileType(
                    file_name.split('.').last().unwrap_or("unknown").to_string(),
                ));
            }
        }

        if !dep.is_empty() {
            Self::dependency(file_ext, file_name, dep);
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn dependency(extension: FileType, file_name: &String, dep: String) {
        let _current_dir: String = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        match extension {
            FileType::Py => {
                python::dependency(&dep);
            }
            FileType::Rs => {
                rust::dep(&dep);
            }
            // FileType::Cpp => {
            //
            //     Command::new("git")
            //         .arg("clone")
            //         .arg(dep.clone())
            //         .arg(format!("{current_dir}/external"))
            //         .status()
            //         .expect("Unable to clone git repository");
            //     let contents = match fs::read_to_string(file_name){
            //         Ok(value) => value,
            //         _ => String::from("#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}")
            //     };
            //     let final_content = format!("#include \"external/{dep}.h\"\n{contents}");
            //     writer(file_name, final_content.as_str());
            //
            // }
            // FileType::C => {
            //     todo!()
            // }
            FileType::Js => {
                javascript::dep(&dep);
            }
            FileType::Ts => {
                typescript::dep(&dep);
            }
            FileType::Go => {
                go::dep(&dep);
            }
            _ => {
                eprintln!("Format not supported for external dependencies");
            }
        }
    }

    pub fn run(file_ext: FileType, file_name: &String) -> Result<(), LionError> {
        let err_creating_target = fs::DirBuilder::new()
            .recursive(true)
            .create("target")
            .is_err();

        if err_creating_target {
            return Err(LionError::Custom(
                "Error while trying to create target folder".to_string(),
            ));
        }

        match file_ext {
            FileType::Go => go::run(file_name),
            FileType::Java => java::run(file_name),
            FileType::Cpp => cpp::run(file_name),
            FileType::C => c::run(file_name),
            FileType::Rs => rust::run(file_name),
            FileType::Py => python::run(file_name),
            FileType::Ts => typescript::run(file_name),
            FileType::Js => javascript::run(file_name),
            FileType::Placeholder => Err(LionError::UnsupportedCommand {
                command: "run".to_string(),
                language: "unknown".to_string(),
            }),
        }
    }

    pub fn project(
        file_ext: FileType,
        proj_name: &String,
        code_file: String,
    ) -> Result<(), LionError> {
        match file_ext {
            FileType::Rs => {
                rust::proj(proj_name);
                init(format!("src/{}", code_file), proj_name.to_owned());
            }

            FileType::Cpp => {
                cpp::proj(proj_name);
                init(format!("src/{}", code_file), proj_name.to_owned());
            }
            FileType::C => {
                //create common directories:
                c::proj(proj_name);
                init(format!("src/{}", code_file), proj_name.to_owned());
            }
            FileType::Go => {
                go::proj(proj_name);
            }
            FileType::Ts => {
                typescript::proj(proj_name)?;
                init(format!("src/{}", code_file), proj_name.to_owned());
            }
            FileType::Placeholder => {
                panic!("Error, unknown or missing file extension");
            }
            FileType::Py => {
                python::proj(proj_name)?;
                init(code_file.clone(), proj_name.to_owned());
            }
            FileType::Java => {
                java::proj(proj_name);
                init(format!("src/{}", code_file), proj_name.to_owned());
            }
            FileType::Js => {
                javascript::proj(proj_name)?;
                init(format!("src/{}", code_file), proj_name.to_owned());
            } // _ => {
              //     panic!("Unsupported Filetype")
              // } // FileType::Html => {}
        }

        //create code file:
        if let Err(err) = Self::new(
            &format!("{proj_name}/src/{code_file}"),
            file_ext,
            String::from(""),
        ) {
            eprintln!("Warning: Failed to create file: {}", err);
        };
        Ok(
            (),
        )
    }
}

fn init(file_path: String, proj_name: String) {
    std::process::Command::new("Lion-cli")
        .current_dir(proj_name)
        .args(["init".to_string(), file_path.to_string()])
        .status()
        .expect("Error initialising project");
}
