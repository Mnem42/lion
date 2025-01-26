use std::env;
mod dependency;
mod runner;
mod util;
mod writer;

struct Input {
    argument1: String,
    argument2: Option<String>,
    argument3: String,
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
        argument1: file_name.unwrap_or(format!("")),
        argument2: ext_dep,
        argument3: file.unwrap_or(String::from("")),
    };

    if args.argument1.to_lowercase() == "help" {
        println!("Help command called.\n{help}");
    } else if args.argument1.contains(".") {
        let extension = args.argument1.split('.').last().unwrap_or("");

        writer::write(extension, &args.argument1, args.argument2);

        println!("Created .{extension} file");
    } else if args.argument1 == "dep" {
        // Only add external dependency
        let extension = args.argument3.split('.').last().unwrap_or("");

        dependency::dependency(extension, &args.argument1, args.argument2);
    } else if args.argument1 == "run" {
        let run_target = args.argument2.unwrap();
        let extension = run_target.split('.').last().unwrap_or("");
        runner::run(extension, &run_target);
    } else {
        println!("Unknown command;\nRun with help to see command list");
    }
}
