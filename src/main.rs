use std::env;
use std::process;

/// Import functions defined on lib.rs to use here.
/// But instead of doing that, we can refer to the module directly.
// use rcat::rcat;
// use rcat::validate_files_existence;

fn main() {
    // Skip the first argument, which is the script name
    let args: Vec<String> = env::args().skip(1).collect();

    // Validates if at least one file is provided
    check_args(&args);

    let response = rcat::cat(&args);
    println!("{}", response);
}

/// At least one file, and it must exist.
fn check_args(args: &[String]) {
    // Check if the provided arguments are empty
    if args.is_empty() {
        usage();

        process::exit(1);
    }

    rcat::validate_files_existence(&args).unwrap_or_else(|err| {
        println!("{}", err);
        usage();
        process::exit(1);
    });
}

/// Prints the usage instructions for the program.
fn usage() {
    eprintln!("Usage: rcat <file1> <file2> ...");
    eprintln!("Concatenates and prints the contents of the specified files.");
}
