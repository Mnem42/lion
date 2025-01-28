use std::env;
mod dependency;
mod runner;
mod util;
mod writer;

struct Input {
    command: String,
    file: String,
    add_ons: String,
}

fn main() {
    let path = env::current_dir().expect("Error");
    println!("The current directory is {}", path.display());
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

    if args.command.to_lowercase() == "help" {
        //
        println!("Help command called.\n{help}");
        //
    } else if args.command.to_lowercase() == "new" {
        //
        let extension = args.file.split('.').last().unwrap_or("");
        writer::write(extension, &args.file, args.add_ons);
        println!("Created .{extension} file");
        //
    } else if args.command.to_lowercase() == "dep" {
        // Only add external dependency
        let extension = args.file.split('.').last().unwrap_or("");
        dependency::dependency(extension, &args.file, args.add_ons.clone());
        //
    } else if args.command.to_lowercase() == "run" {
        //
        let run_target = args.file;
        let extension = run_target.split('.').last().unwrap_or("");
        runner::run(extension, &run_target);
        //
    } else {
        println!("Unknown command;\nRun with 'help' to see command list");
    }
}
