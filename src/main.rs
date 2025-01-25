use std::env;
mod writer;

struct Input {
    filename: String,
    dependency: Option<String>,
}

fn main() {
    let help =
        "Command list:\n lion <fileName.extension> -> Creates a file with filler code of the extension type\n";
    let file_name = env::args()
        .nth(1)
        .expect("No file name given.\nPlease provide a file name and try again\nRun lion help for the list of commands\n");
    let ext_dep = env::args().nth(2);

    let args = Input {
        filename: file_name,
        dependency: ext_dep,
    };

    if args.filename.to_lowercase() == "help" {
        println!("Help command called.\n{help}");
        return;
    }
    let extension = args.filename.split('.').last().unwrap_or("");
    writer::write(extension, &args.filename, args.dependency);
    println!("Created .{extension} file");
}
