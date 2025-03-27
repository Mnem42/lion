use std::env;

mod controller;
mod languages;
mod utils;

use controller::{FileType, Language};
use setter::config::Config;
use utils::config_file_exists;

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

    let setting = Config {
        setting: "file_ext".to_string(),
        mode: "".to_string(),
        file: "Lion.toml".to_string(),
        divider: "[Project]".to_string(),
    };

    let first_arg = env::args().nth(1);
    let mut second_arg = env::args().nth(2);
    let third_arg = env::args().nth(3);

    if second_arg.clone().unwrap_or(String::new()).is_empty() {
        second_arg = setting.read_config("file_path", "[Project]");
    }

    let args = Input {
        command: first_arg.unwrap_or_default(),
        file: second_arg.unwrap_or_default(),
        add_ons: third_arg.unwrap_or_default(),
    };

    let extension = args.file.split('.').last().unwrap_or("");

    // Match the file extension to determine the `FileType`
    let mut file_ext = match extension {
        "cpp" => FileType::Cpp,
        "rs" => FileType::Rs,
        "c" => FileType::C,
        "go" => FileType::Go,
        "py" => FileType::Py,
        "java" => FileType::Java,
        "ts" => FileType::Ts,
        "js" => FileType::Js,
        _ => FileType::Placeholder,
    };

    if config_file_exists() {
        let set_file_ext = match setting
            .read_config("file_ext", "[Project]")
            .unwrap_or("".to_string())
            .as_str()
        {
            "cpp" => FileType::Cpp,
            "rs" => FileType::Rs,
            "c" => FileType::C,
            "go" => FileType::Go,
            "py" => FileType::Py,
            "java" => FileType::Java,
            "ts" => FileType::Ts,
            "js" => FileType::Js,
            _ => FileType::Placeholder,
        };

        if set_file_ext != file_ext {
            println!("WARNING: File provided is of different type as set type.")
        }
        file_ext = set_file_ext;
    } else if args.command.to_lowercase().as_str() != "proj" {
        utils::file_setup(&args.file, extension.to_string());
    }

    let command_base = controller::Language {
        file_extension: file_ext,
    };

    match args.command.to_lowercase().as_str() {
        "new" => {
            Language::new(&args.file, command_base.file_extension, args.add_ons);
            println!("Created .{extension} file");
        }
        "help" => {
            println!("Help command called.\n{help}");
        }
        "dep" => {
            Language::dependency(
                command_base.file_extension,
                &args.file,
                args.add_ons.clone(),
            );
        }
        "run" => {
            Language::run(command_base.file_extension, &args.file);
        }
        "proj" => {
            Language::project(
                command_base.file_extension,
                &args.add_ons,
                args.file.clone(),
            );
            // let mut file = Config {
            //     setting: "file_ext".to_string(),
            //     mode: extension.to_string(),
            //     file: format!("{}/Lion.toml", args.add_ons),
            //     divider: "[Project]".to_string(),
            // };
            // file.init();
            // file.setting = String::from("file_path");
            // file.mode = args.add_ons;
            // file.write_config();
            std::process::Command::new("Lion-cli")
                .current_dir(args.add_ons)
                .args(["init".to_string(), format!("src/{}", args.file)])
                .status()
                .expect("Error initialising project");
        }
        "init" => {
            utils::file_setup(&args.file, extension.to_string());
        }
        _ => eprintln!("Unknown command;\nRun with 'help' to see command list"),
    }
}
