use std::{env, process};
use rcat::{usage, cat};

use rcat::Arguments;

fn main() {
    // Skip the first argument, which is the script name
    // and sanitize the rest of them.
    let args: Vec<String> = env::args().skip(1).collect();
    let arguments:Arguments = Arguments::new(&args);

    // Check if the user has provided the help option.
    if arguments.has_opion(String::from("-h")) || arguments.has_opion(String::from("--help")) {
        eprintln!("\n");
        usage();
        process::exit(0);
    }

    match cat(arguments) {
        Ok(response) => {
            println!("{}", response);
        }
        Err(err) => {
            eprintln!("\nRuntime Error: {}\n", err);
            process::exit(1);
        }
    }
}
