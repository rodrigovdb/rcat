use std::env;

/// Import functions defined on lib.rs
use rcat::rcat;
use rcat::validate_files_existence;

fn main() {
    // Skip the first argument, which is the script name
    let args: Vec<String> = env::args().skip(1).collect();

    // Validates if at least one file is provided
    check_args(&args);

    let response = rcat(&args);
    println!("{}", response);
}

/// At least one file, and it must exist.
fn check_args(args: &[String]) {
    // Check if the provided arguments are empty
    if args.is_empty() {
        usage();

        std::process::exit(1);
    }

    // Check if the provided arguments are existent files
    if let Err(err) = std::panic::catch_unwind(|| validate_files_existence(&args)) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

/// Prints the usage instructions for the program.
fn usage() {
    eprintln!("Usage: rcat <file1> <file2> ...");
    eprintln!("Concatenates and prints the contents of the specified files.");
}
