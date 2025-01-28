use std::fs;

pub fn file_creator(file_name: &String, file_content: &str) {
    fs::write(file_name, file_content).expect("An Unexpected error occured; please try again!");
}
