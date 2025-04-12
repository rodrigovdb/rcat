use std::fs;
use std::error::Error;

pub fn ensure_valid_args(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("No file(s) provided.".into());
    }

    for file in args {
        if !fs::metadata(file).is_ok() {
            return Err(format!("File {} does not exist", file).into());
        }
    }

    return Ok(());
}

pub fn usage() {
    eprintln!("Usage: rcat <file1> <file2> ...");
    eprintln!("Concatenates and prints the contents of the specified files.");
}

pub fn cat(args: &[String]) -> Result<String, Box<dyn Error>> {
    let mut response = String::new();

    for file in args {
        match fs::read_to_string(file) {
            Ok(content) => response.push_str(&content),
            Err(e) => return Err(format!("Error reading file {}: {}", file, e).into()),
        }
    }

    return Ok(response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_validating_inexistent_file() {
        let args = vec![
            String::from("fixtures/file_three")
        ];

        let result = validate_files_existence(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File fixtures/file_three does not exist");
    }

    #[test]
    fn rcat_single_file() {
        let args = vec![
            String::from("fixtures/file_one")
        ];
        
        let result = cat(&args);

        let expected = "First File\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn rcat_two_files() {
        let args = vec![
            String::from("fixtures/file_one"),
            String::from("fixtures/file_two")
        ];
        
        let result = cat(&args);

        let expected = "First File\nSecond File\n";
        assert_eq!(result, expected);
    }
}
