use std::env;

mod controller;
mod languages;
mod utils;

use controller::{FileType, Language};
use setter::config::Config;

struct Input {
    command: String,
    file: String,
    add_ons: String,
}

fn main() {
    let help =
        "Command list:\n
        lion-cli new <fileName.extension> -> Creates a file with filler code of the extension type\n
        lion-cli new <fileName.extension> <dependency> -> Adds an external dependency and creates a file with the provided file name\n      (Only supported for python, rust and go)\n
        lion-cli dep <dependency> <fileName.extension> -> adds the respective dependency to the file\n
        lion-cli run <fileName.extension> -> runs the file specified (see the docs on supported languages)\n
        lion-cli proj <fileName.extesnion> <project_name> -> Creates a project with the specified name";

    let first_arg = env::args().nth(1);
    let second_arg = env::args().nth(2);
    let third_arg = env::args().nth(3);

    let args = Input {
        command: first_arg.unwrap_or_default(),
        file: second_arg.unwrap_or_default(),
        add_ons: third_arg.unwrap_or_default(),
    };

    let extension = args.file.split('.').last().unwrap_or("");

    // Match the file extension to determine the `FileType`
    let file_ext = match extension {
        "cpp" => FileType::Cpp,
        "rs" => FileType::Rs,
        "c" => FileType::C,
        "go" => FileType::Go,
        "py" => FileType::Py,
        "java" => FileType::Java,
        "ts" => FileType::Ts,
        _ => FileType::Placeholder,
    };

    let mut command_base = controller::Language {
        file_extension: file_ext,
        dependency_file: String::from(""),
        command: controller::MyCommand::Empty,
    };

    match args.command.to_lowercase().as_str() {
        "new" => {
            command_base.command = controller::MyCommand::New;
            Language::new(&args.file, command_base.file_extension, args.add_ons);
            println!("Created .{extension} file");
        }
        "help" => {
            command_base.command = controller::MyCommand::Help;
            println!("Help command called.\n{help}");
        }
        "dep" => {
            command_base.command = controller::MyCommand::Dep;
            Language::dependency(
                command_base.file_extension,
                &args.file,
                args.add_ons.clone(),
            );
        }
        "run" => {
            command_base.command = controller::MyCommand::Run;
            Language::run(command_base.file_extension, &args.file);
        }
        "proj" => {
            command_base.command = controller::MyCommand::Proj;
            Language::project(command_base.file_extension, &args.add_ons, args.file);
        }
        "init" => {
            let file = Config {
                setting: "file_ext".to_string(),
                mode: extension.to_string(),
                file: "Lion.toml".to_string(),
                divider: "[Project]".to_string(),
            };
            file.init();
        }
        _ => eprintln!("Unknown command;\nRun with 'help' to see command list"),
    }
}
