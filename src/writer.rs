use crate::dependency;
use crate::util;

pub fn write(file_ext: &str, file_name: &String, dependency: Option<String>) {
    if let Some(dependency_string) = dependency {
        dependency::dependency(file_ext, file_name, dependency_string);
    } else {
        match file_ext {
            "py" => util::file_creator(file_name, "print(\"Hello Lion!\")"),

            "rs" => util::file_creator(file_name, "fn main() {\n    println!(\"Hello Lion!\");\n}"),
            "cpp" => util::file_creator(
                file_name,
                "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, Lion!\" << std::endl;\n    return 0;\n}",
            ),
            "c" => util::file_creator(file_name, "#include <stdio.h>

            int main() {
                printf(\"Hello Lion!\");
                return 0;
            }"),
            "go" => util::file_creator(
                file_name,
                "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello Lion!\")\n}"),
            "java" => util::file_creator(file_name, "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, Lion!\");\n    }\n}"),
            _ => panic!("Unsupported file format or an error occured!"),
        }
    }
}
