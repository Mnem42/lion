use std::fs;

pub fn write(file_ext: &str, file_name: &String) {
    match file_ext {
        "py" => {
            fs::write(file_name, r#"print("Hello Lion!")"#).expect("Unable to create file");
        }
        "rs" => {
            fs::write(
                file_name,
                r#"fn main() {
    println!("Hello Lion!");
}"#,
            )
            .expect("Unable to create file");
        }
        "cpp" => fs::write(
            file_name,
            r#"#include <iostream>

int main() {
    std::cout << "Hello, Lion!" << std::endl;
    return 0;
}"#,
        )
        .expect("Unable to create file"),
        _ => panic!("Unsupported file format or an error occured!"),
    }
}
