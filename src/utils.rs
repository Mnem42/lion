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
