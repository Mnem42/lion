use crate::util;
use std::fs;

pub fn dependency(extension: &str, file_name: &String, dep: String) {
    match extension {
        "py" => {
            let contents = match fs::read_to_string(file_name) {
                Ok(value) => value,
                Err(_) => "\nprint(\"Hello Lion!\")".to_string(),
            };
            util::file_creator(file_name, format!("import {dep}\n{contents}").as_str())
        }
        "rs" => {
            let file_contents = match fs::read_to_string("Cargo.toml") {
                Ok(value) => value,
                Err(_) => {
                    util::file_creator(&String::from("Cargo.toml"), "[dependencies]");
                    String::from("[dependencies]")
                }
            };
            let Some((before, after)) = file_contents.split_once("[dependencies]") else {
                panic!("No `[dependencies]` field in your Cargo.toml");
            };

            let final_content = format!("{}[dependencies]\n{} = \"*\"{}\n", before, dep, after);
            util::file_creator(&String::from("Cargo.toml"), final_content.as_str());
            util::file_creator(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}");
        }
        "cpp" => {
            let contents = match fs::read_to_string(file_name){
                Ok(value) => value,
                _ => String::from("#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}")
            };
            let final_content = format!("#include \"{dep}/{dep}.h\"\n{contents}");
            util::file_creator(file_name, final_content.as_str());
        }

        _ => {
            eprintln!("Format not supported for external dependencies");
        }
    }
}
