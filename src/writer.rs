use std::fs;

fn file_creator(file_name: &String, file_content: &str) {
    fs::write(file_name, file_content).expect("An Unexpeceted error occured, please try again!");
}

pub fn write(file_ext: &str, file_name: &String) {
    match file_ext {
        "py" => file_creator(file_name, r#"print("Hello Lion!")"#),

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
        _ => panic!("Unsupported file format or an error occured!"),
    }
}
