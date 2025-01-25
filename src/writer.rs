use std::fs;

fn file_creator(file_name: &String, file_content: &str) {
    fs::write(file_name, file_content).expect("An Unexpeceted error occured, please try again!");
}

pub fn write(file_ext: &str, file_name: &String, dependency: Option<String>) {
    if let Some(_) = dependency {
        let dependency_string = dependency.unwrap();
        match file_ext {
            "py" => file_creator(
                file_name,
                format!("import {dependency_string}\n\nprint(\"Hello Lion!\")").as_str(),
            ),
            "rs" => {
                let file_contents = fs::read_to_string("Cargo.toml".to_string())
                    .expect("No Cargo.toml found, please create one to add a dependency");
                let vector: Vec<&str> = file_contents.split("[dependencies]").collect();

                let final_content = format!(
                    "{}[dependencies]\n{} = \"*\"{}\n",
                    vector[0], dependency_string, vector[1]
                );
                file_creator(&String::from("Cargo.toml"), final_content.as_str());
                file_creator(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}");
            }
            _ => {}
        }
    } else {
        match file_ext {
            "py" => file_creator(file_name, "print(\"Hello Lion!\")"),

            "rs" => file_creator(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}"),
            "cpp" => file_creator(
                file_name,
                "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}",
            ),
            "c" => file_creator(file_name, "#include <stdio.h>

            int main() {
                printf(\"Hello Lion!\");
                return 0;
            }"),
            "go" => file_creator(
                file_name,
                "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello Lion!\")\n}"),
            "java" => file_creator(file_name, "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, Lion!\");\n    }\n}"),
            _ => panic!("Unsupported file format or an error occured!"),
        }
    }
}
