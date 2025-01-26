use std::process::Command;

pub fn run(file_ext: &str, file_name: &String) {
    match file_ext {
        "cpp" => {
            let output = {
                Command::new("g++")
                    .arg(file_name)
                    .arg("-o")
                    .arg(file_name)
                    .spawn()
                    .expect("An error occured; Please try again.");
                Command::new(format!("./{file_name}"))
                    .spawn()
                    .expect("An error occured; Please try again.")
            };
            println!("Running");
            output.stdout;
        }
        "rs" => {
            let output = if cfg!(target_os = "windows") {
                Command::new("rustc")
                    .arg(file_name)
                    .output()
                    .expect("An error occured; Please try again.");
                Command::new(format!(".\\{file_name}.exe"))
                    .output()
                    .expect("An error occured; Please try again.")
            } else {
                Command::new("rustc")
                    .arg(file_name)
                    .output()
                    .expect("An error occured; Please try again.");
                Command::new(format!("./{file_name}"))
                    .output()
                    .expect("An error occured; Please try again.")
            };
            output.stdout;
        }
        _ => {
            todo!()
        }
    }
}
