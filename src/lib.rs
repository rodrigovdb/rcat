use std::fs;
use std::error::Error;

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

pub fn cat(arguments: Arguments) -> Result<String, Box<dyn Error>> {
    let mut response = String::new();

    for file in arguments.files() {
        match fs::read_to_string(file) {
            Ok(content) => response.push_str(&content),
            Err(e) => return Err(format!("Error reading file {}: {}", file, e).into()),
        }
    }

    return Ok(response);
}
