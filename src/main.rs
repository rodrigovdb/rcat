use std::{env, process};
use rcat::{ensure_valid_args, usage, cat};

fn main() {
    // Skip the first argument, which is the script name
    let args: Vec<String> = env::args().skip(1).collect();

    // Validates if at least one file is provided
    if let Err(err) = ensure_valid_args(&args) {
        eprintln!("\nERROR: {}\n", err);
        usage();
        process::exit(1);
    }

    match cat(&args) {
        Ok(response) => {
            println!("{}", response);
        }
        Err(err) => {
            dbg!(&err);
            process::exit(1);
        }
    }
}
