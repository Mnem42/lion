use crate::util;
use std::fs;

pub fn dependency(file_ext: &str, file_name: &String, dep: Option<String>) {
    let dep = dep.unwrap();
    match file_ext {
        "py" => util::file_creator(
            file_name,
            format!("import {dep}\n\nprint(\"Hello Lion!\")").as_str(),
        ),
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
        }
        _ => {
            eprintln!("Format not supported for external dependencies");
        }
    }
}
