use crate::errors::{Result, file_io_error};
use setter::config::Config;
use std::fs;

pub fn writer(file_name: &String, file_contents: &str) -> Result<()> {
    fs::write(file_name, file_contents).map_err(|err| file_io_error(file_name, "writing to", err))
}

pub fn common_dir(proj_name: &String) -> Result<()> {
    // Create project folder
    fs::DirBuilder::new()
        .recursive(true)
        .create(proj_name)
        .map_err(|err| file_io_error(proj_name, "creating directory", err))?;

    // Create src folder inside project directory
    let src_path = format!("{proj_name}/src");
    fs::DirBuilder::new()
        .recursive(true)
        .create(&src_path)
        .map_err(|err| file_io_error(&src_path, "creating directory", err))?;

    // Create target folder
    let target_path = format!("{proj_name}/target");
    fs::DirBuilder::new()
        .recursive(true)
        .create(&target_path)
        .map_err(|err| file_io_error(&target_path, "creating directory", err))?;

    writer(&format!("{proj_name}/.gitignore"), "/target")?;

    Ok(())
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
