use std::env;
mod dependency;
mod runner;
mod util;
mod writer;

struct Input {
    argument1: String,
    argument2: Option<String>,
}

fn main() {
    let help =
        "Command list:\n
        lion <fileName.extension> -> Creates a file with filler code of the extension type\n
        lion <fileName.extension> <dependency> -> Adds an external dependency and creates a file with the provided file name\n";
    let file_name = env::args()
        .nth(1)
        .expect("No file name given.\nPlease provide a file name and try again\nRun lion help for the list of commands\n");
    let ext_dep = env::args().nth(2);

    let args = Input {
        argument1: file_name,
        argument2: ext_dep,
    };
    let extension = args.argument1.split('.').last().unwrap_or("");

    if args.argument1.to_lowercase() == "help" {
        println!("Help command called.\n{help}");
        return;
    } else if args.argument1.contains(".") {
        writer::write(extension, &args.argument1, args.argument2);
        println!("Created .{extension} file");
    } else if args.argument1 == "dep" {
        dependency::dependency(extension, &args.argument1, args.argument2);
    }
}
