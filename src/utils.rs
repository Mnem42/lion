use setter::config::Config;
use std::fs;

pub fn writer(file_name: &String, file_contents: &str) {
    if let Err(error) = fs::write(file_name, file_contents) {
        panic!("An error occured:\n{error}")
    }
}

pub fn common_dir(proj_name: &String) {
    //create project folder
    if let Err(error) = fs::DirBuilder::new().recursive(true).create(proj_name) {
        panic!("error: {error}")
    }

    //Create src folder inside project directory
    if let Err(error) = fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/src"))
    {
        panic!("error: {error}")
    }

    //create target folder:
    if let Err(error) = fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{proj_name}/target"))
    {
        panic!("error: {error}")
    }
    writer(&format!("{proj_name}/.gitignore"), "/target");
}

pub fn file_setup(file_path: &String, extension: String) {
    let mut file = Config {
        setting: "file_ext".to_string(),
        mode: extension.to_string(),
        file: "Lion.toml".to_string(),
        divider: "[Project]".to_string(),
    };
    file.init();
    file.setting = String::from("file_path");
    file.mode = file_path.to_owned();
    file.write_config();
}

pub fn config_file_exists() -> bool {
    std::path::Path::new("Lion.toml").exists()
}
