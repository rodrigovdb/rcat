use std::env;
use std::fs;

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
    for file in args {
        if !fs::metadata(file).is_ok() {
            eprintln!("File {} does not exist.", file);
            std::process::exit(1);
        }
    }
}

/// Prints the usage instructions for the program.
fn usage() {
    eprintln!("Usage: rcat <file1> <file2> ...");
    eprintln!("Concatenates and prints the contents of the specified files.");
}

/// Reads the contents of the files and concatenates them into a single string.
fn rcat(args: &[String]) -> String {
    // This is the response, we'll accumulate the content of all files here
    let mut response = String::new();

    // Iterate over each file provided in the arguments and add to the accumulated response
    for file in &args[0..] {
        match fs::read_to_string(file) {
            Ok(content) => response.push_str(&content),
            Err(e) => eprintln!("Error reading file {}: {}", file, e),
        }
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_args_success() {
        let args = vec![
            String::from("fixtures/file_one"),
            String::from("fixtures/file_two"),
        ];

        // Ensure check_args does not panic for valid arguments
        check_args(&args);
    }

    #[test]
    fn rcat_success() {
        let args = vec![
            String::from("fixtures/file_one"),
            String::from("fixtures/file_two"),
        ];
        
        let result = rcat(&args);

        let expected = "First File\nSecond File\n";
        assert_eq!(result, expected);
    }
}
