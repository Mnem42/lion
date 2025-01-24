use std::{env, fs};

struct Input {
    file_name: String,
}

fn main() {
    let help = "Enter file name with language extension";
    let file_name = env::args().nth(1).expect("no pattern given");
    // let _ = env::args().nth(2).expect("no path given");

    let args = Input {
        file_name: file_name,
    };

    if args.file_name.to_lowercase() == "help" {
        println!("Help command called.\n{help}");
        return;
    }
    let extension = args.file_name.split('.').last().unwrap_or("");

    match extension {
        "py" => {
            fs::write(args.file_name, r#"print("Hello Lion!")"#).expect("Unable to create file");
        }
        "rs" => {
            fs::write(
                args.file_name,
                r#"fn main() {
    println!("Hello Lion!");
}"#,
            )
            .expect("Unable to create file");
        }
        "cpp" => fs::write(
            args.file_name,
            r#"#include <iostream>

int main() {
    std::cout << "Hello, Lion!" << std::endl;
    return 0;
}"#,
        )
        .expect("Unable to create file"),
        _ => eprintln!("An error occured"),
    }
}
