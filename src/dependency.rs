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
            let file_contents = fs::read_to_string("Cargo.toml")
                .expect("No Cargo.toml found; please create one to add a dependency");
            let Some((before, after)) = file_contents.split_once("[dependencies]") else {
                panic!("No [dependencies] field in your Cargo.toml");
            };

            let final_content = format!("{}[dependencies]\n{} = \"*\"{}\n", before, dep, after);
            util::file_creator(&String::from("Cargo.toml"), final_content.as_str());
            util::file_creator(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}");
        }
        _ => {
            eprintln!("Format not supported for external dependencies");
        }
    }
}
