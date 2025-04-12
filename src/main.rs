use std::{env, process};
use rcat::{usage, cat};

use rcat::Arguments;

fn main() {
    // Skip the first argument, which is the script name
    // and sanitize the rest of them.
    let args: Vec<String> = env::args().skip(1).collect();
    let arguments:Arguments = match Arguments::new(&args) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("\nError: {}\n", err);
            usage();
            process::exit(1);
        }
    };

    // Check if the user has provided the help option.
    if arguments.has_option("-h") || arguments.has_option("--help") {
        eprintln!("\n");
        usage();
        process::exit(0);
    }

    // Run the cat thing.
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
