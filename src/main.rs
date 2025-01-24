use std::env;
mod writer;

struct Input {
    file_name: String,
}

fn main() {
    let help =
        "Command list:\n lion <fileName.extension> -> Creates a file with filler code of the extension type\n";
    let file_name = env::args()
        .nth(1)
        .expect("No file name given.\nPlease provide a file name and try again");
    //let _ext_dep = env::args().nth(2).expect("no path given");

    let args = Input {
        file_name: file_name,
    };

    if args.file_name.to_lowercase() == "help" {
        println!("Help command called.\n{help}");
        return;
    }
    let extension = args.file_name.split('.').last().unwrap_or("");
    writer::write(extension, &args.file_name);
    println!("Created .{extension} file");
}
