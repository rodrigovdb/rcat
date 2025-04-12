use std::collections::HashMap;

use std::error::Error;

// use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;

// #[derive(Debug)]
pub struct Arguments {
    options: Vec<String>,
    files: Vec<String>,
}
impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, String> {
        let mut options = Vec::new();
        let mut files = Vec::new();

        for arg in args {
            if arg.starts_with('-') {
                if Arguments::available_options().iter().any(|(opt, _)| opt == arg) {
                    options.push(arg.clone());
                } else {
                    return Err(format!("Invalid option: {}", arg));
                }
            } else {
                files.push(arg.clone());
            }
        }

        Ok(Arguments { options, files })
    }
    
    pub fn has_option(&self, option: &str) -> bool {
        self.options.contains(&option.to_string())
    }

    pub fn files(&self) -> &Vec<String> {
        &self.files
    }

    pub fn available_options() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("-E", "Display a `$` at the end of each line"),
            ("-n", "Number all output lines"),
            ("-T", "Display tab characters as `^I`"),
            ("-l", "Adds an empty line between each file"),
            ("-h", "Show this help"),
        ])
    }
}

pub fn usage() {
    eprintln!("Concatenates and prints the contents of the specified files.");
    eprintln!("Usage: rcat [options] file1 [<file2> ... <fileN>]");

    eprintln!("\nOptions:");
    for (option, description) in Arguments::available_options() {
        eprintln!("  {}: {}", option, description);
    }

    eprintln!("\nExamples:");
    eprintln!("  rcat file1.txt");
    eprintln!("  rcat -l -E file1.txt file2.txt");
    eprintln!("  rcat -T -l file1.txt file2.txt > output.txt");

    eprintln!("\n");
}

/**
 * Receives an instance of Arguments, which has the files to be read.
 * It parses each one and returns the content as a string.
 * If an error occurs, it returns an error message.
 */
pub fn cat(arguments: Arguments) -> Result<String, Box<dyn Error>> {
    let mut response = String::new();

    let mut first = true;
    let mut count:usize = 1;

    for filepath in arguments.files() {
        // Adds an empty line beween each file.
        if arguments.has_option("-l") && !first {
            // If we are counting lines, we need to add the line number.
            if arguments.has_option("-n") {
                response.push_str(&format!("{} ", count));
                count += 1;
            }

            response.push_str("\n");
        }

        match parse_file(filepath, &arguments, &mut count) {
            Ok(content) => response.push_str(&content),
            Err(e) => return Err(format!("Error reading file {}: {}", filepath, e).into()),
        }

        first = false
    }

    return Ok(response);
}

/**
 * Get a file path and open the file.
 * Read the file line by line and return the content as a string.
 */
 fn parse_file(filepath: &String, arguments: &Arguments, count: &mut usize) -> Result<String, Box<dyn Error>> {
    let mut response = String::new();

    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let parsed_line = parse_line(line?, &arguments, count);
        response.push_str(parsed_line.as_str());
    }

    Ok(response)
}

/**
 * Receives a line and an instance of Arguments.
 * Also receives a mutable reference to a counter to update the line number.
 * It parses the line according to the options provided in Arguments.
 * Returns the parsed line.
 */
fn parse_line(line: String, arguments: &Arguments, count: &mut usize) -> String {
    let mut response = String::new();

    // Add the line number at the beginning of the line, when the -n option is used.
    if arguments.has_option("-n") {
        response.push_str(&format!("{} ", count));
        *count += 1;
    }

    // Replace \t with ^I when the -T option is used.
    if arguments.has_option("-T") {
        response = line.replace("\t", "^I");
    } else {
        response.push_str(&line);
    }

    // Add the $ char to the end of the line, when the -E option is used.
    if arguments.has_option("-E") {
        response.push_str("$");
    }

    response.push_str("\n");

    response
}
