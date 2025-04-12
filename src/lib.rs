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
    pub fn new(args: &[String]) -> Arguments {
        let mut options = Vec::new();
        let mut files = Vec::new();

        for arg in args {
            if arg.starts_with('-') {
                options.push(arg.clone());
            } else {
                files.push(arg.clone());
            }
        }

        Arguments { options, files }
    }
    
    pub fn has_opion(&self, option: String) -> bool {
        self.options.contains(&option)
    }

    pub fn files(&self) -> &Vec<String> {
        &self.files
    }

    pub fn available_options() -> Vec<(&'static str, &'static str)> {
        vec![
            ("-n", "Display line numbers"),
            ("-E", "Display a `$` at the end of each line"),
            ("-T", "Display tab characters as `^I`"),
            ("-h", "Show this help"),
        ]
    }
}

pub fn usage() {
    eprintln!("Concatenates and prints the contents of the specified files.");
    eprintln!("Usage: rcat [options] <file1> <file2> ...");
    eprintln!("\nOptions:");
    for (option, description) in Arguments::available_options() {
        eprintln!("  {}: {}", option, description);
    }
    eprintln!("\n");
}

/**
 * Receives an instance of Arguments, which has the files to be read.
 * It parses each one and returns the content as a string.
 * If an error occurs, it returns an error message.
 */
pub fn cat(arguments: Arguments) -> Result<String, Box<dyn Error>> {
    let mut response = String::new();

    for filepath in arguments.files() {
        match parse_file(filepath) {
            Ok(content) => response.push_str(&content),
            Err(e) => return Err(format!("Error reading file {}: {}", filepath, e).into()),
        }
    }

    return Ok(response);
}

/**
 * Get a file path and open the file.
 * Read the file line by line and return the content as a string.
 */
fn parse_file(filepath: &String) -> Result<String, Box<dyn Error>> {
    let mut response = String::new();

    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        response.push_str(&line);
        response.push('\n');
    }

    Ok(response)
}
