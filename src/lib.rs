use std::fs;

pub fn validate_files_existence(args: &[String]) -> Result<(), &'static str> {
    // Check if the provided arguments are existent files
    for file in args {
        if !fs::metadata(file).is_ok() {
            let msg = format!("File {} does not exist", file);

            return Err(Box::leak(msg.into_boxed_str()));
        }
    }

    return Ok(());
}

/// Reads the contents of the files and concatenates them into a single string.
pub fn cat(args: &[String]) -> String {
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
