use std::process::Command;

pub fn run(file_ext: &str, file_name: &String) {
    match file_ext {
        "cpp" => {
            let _ = Command::new("g++")
                .arg("{file_name}")
                .arg("-o")
                .arg("output")
                .output()
                .expect("An error occured; Please try again.");
            println!("Running you")
        }
        "rs" => {
            let _ = Command::new("rustc")
                .arg(file_name)
                .output()
                .expect("An error occured; Please try again.");
            if cfg!(target_os = "windows") {
                Command::new("rustc")
                    .arg(format!("{file_name}.exe"))
                    .output()
                    .expect("An error occured; Please try again.")
            } else {
                Command::new("rustc")
                    .arg(file_name)
                    .output()
                    .expect("An error occured; Please try again.")
            };
        }
        _ => {
            todo!()
        }
    }
}
