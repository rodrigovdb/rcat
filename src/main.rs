use std::env;
use std::fs;

fn main() {
    // Skip the first argument, which is the script name
    let args: Vec<String> = env::args().skip(1).collect();

    // Validates if at least one file is provided
    check_args(&args);

    // This is the response, we'll accumulate the content of all files here
    let mut response = String::new();

    // Iterate over each file provided in the arguments and add to the accumulated response
    for file in &args[0..] {
        match fs::read_to_string(file) {
            Ok(content) => response.push_str(&content),
            Err(e) => eprintln!("Error reading file {}: {}", file, e),
        }
    }

    println!("{}", response);
}

fn check_args(args: &[String]) {
    // Check if the provided arguments are empty
    if args.is_empty() {
        eprintln!("Not enough arguments provided.");
        eprintln!("Provided arguments: {:?}", args);

        std::process::exit(1);
    }

    // Check if the provided arguments are existent files
    for file in args {
        if !fs::metadata(file).is_ok() {
            eprintln!("File {} does not exist.", file);
            std::process::exit(1);
        }
    }
}
