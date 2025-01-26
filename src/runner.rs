use std::process::Command;

pub fn run(file_ext: &str, file_name: &String) {
    match file_ext {
        "cpp" => {
            Command::new("g++")
                .arg(file_name)
                .arg("-o")
                .arg("lion_compiled")
                .status()
                .expect("An error occured; Please try again.");
            println!("Compiled...");
            Command::new(format!("./lion_compiled"))
                .status()
                .expect("An error occured; Please try again.");
            println!("Ran the code successfully");
        }
        "rs" => {
            if cfg!(target_os = "windows") {
                Command::new("rustc")
                    .arg(file_name)
                    .args(["-o", "lion_compiled"])
                    .status()
                    .expect("An error occured; Please try again.");
                println!("Compiled...");
                Command::new(format!(".\\lion_compiled.exe"))
                    .status()
                    .expect("An error occured; Please try again.");
                println!("Ran the code successfully");
            } else {
                Command::new("rustc")
                    .arg(file_name)
                    .args(["-o", "lion_compiled"])
                    .status()
                    .expect("An error occured; Please try again.");
                println!("Compiled...");
                Command::new(format!("./lion_compiled"))
                    .status()
                    .expect("An error occured; Please try again.");
                println!("Ran the code successfully");
            };
        }
        _ => {
            todo!()
        }
    }
}
