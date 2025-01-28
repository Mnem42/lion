use std::env;

use languages::{FileType, Functions, Language, MyCommand};

mod dependency;
mod languages;
mod runner;
mod util;
mod writer;

struct Input {
    command: String,
    file: String,
    add_ons: String,
}

fn main() {
    let help =
        "Command list:\n
        lion-cli <fileName.extension> -> Creates a file with filler code of the extension type\n
        lion-cli <fileName.extension> <dependency> -> Adds an external dependency and creates a file with the provided file name\n
        lion-cli dep <dependency> <fileName.extension> -> adds the respective dependency to the file\n
        lion-cli run <fileName.extension> -> runs the file specified (see the docs on supported languages)\n";

    let file_name = env::args().nth(1);
    let ext_dep = env::args().nth(2);
    let file = env::args().nth(3);

    let args = Input {
        command: file_name.unwrap_or(String::new()),
        file: ext_dep.unwrap_or(String::new()),
        add_ons: file.unwrap_or(String::new()),
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
        _ => FileType::Placeholder,
    };

    let mut command_base = languages::Language {
        file_extension: file_ext,
        dependency_file: String::from(""),
        command: languages::MyCommand::Empty,
    };

    if args.command.to_lowercase() == "help" {
        //
        println!("Help command called.\n{help}");
        command_base.command = languages::MyCommand::Help;
        //
    } else if args.command.to_lowercase() == "new" {
        //
        command_base.command = languages::MyCommand::New;
        Language::new(&args.file, command_base.file_extension);
        println!("Created .{extension} file");
        //
    } else if args.command.to_lowercase() == "dep" {
        // Only add external dependency
        command_base.command = languages::MyCommand::Dep;
        Language::dependency(
            command_base.file_extension,
            &args.file,
            args.add_ons.clone(),
            String::from(""),
        );
        //
    } else if args.command.to_lowercase() == "run" {
        //
        command_base.command = languages::MyCommand::Run;
        Language::run(command_base.file_extension, &args.file);
        //
    } else {
        println!("Unknown command;\nRun with 'help' to see command list");
    }
}
