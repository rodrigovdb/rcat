use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::io::AsRawFd;
use std::process;

fn usage() {
    eprintln!(
        "Usage: rcat [OPTION]... [FILE]...\n\
         Concatenate FILE(s) to standard output.\n\n\
         Options:\n\
         -A, --show-all           equivalent to -vET\n\
         -b, --number-nonblank    number nonempty output lines, overrides -n\n\
         -e                       equivalent to -vE\n\
         -E, --show-ends          display $ at end of each line\n\
         -n, --number             number all output lines\n\
         -s, --squeeze-blank      suppress repeated empty output lines\n\
         -t                       equivalent to -vT\n\
         -T, --show-tabs          display TAB characters as ^I\n\
         -u                       (ignored)\n\
         -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB\n"
    );
    process::exit(1);
}

fn next_line_num(line_num: &mut usize) -> String {
    *line_num += 1;
    format!("{:6}\t", line_num)
}

fn cat_file(
    file: &str,
    options: &CatOptions,
    line_num: &mut usize,
    writer: &mut impl Write,
) -> io::Result<()> {
    let file: Box<dyn Read> = if file == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(file)?)
    };

    let reader = BufReader::new(file);
    let mut last_line_empty = false;

    for line in reader.lines() {
        let mut line = line?;
        let is_empty = line.is_empty();

        if options.squeeze_blank && is_empty && last_line_empty {
            continue;
        }

        if options.number && (!options.number_nonblank || !is_empty) {
            write!(writer, "{}", next_line_num(line_num))?;
        }

        if options.show_tabs {
            line = line.replace('\t', "^I");
        }

        write!(writer, "{}", line)?;

        if options.show_ends {
            write!(writer, "$")?;
        }

        writeln!(writer)?;

        last_line_empty = is_empty;
    }

    Ok(())
}

struct CatOptions {
    number: bool,
    number_nonblank: bool,
    squeeze_blank: bool,
    show_ends: bool,
    show_tabs: bool,
    show_nonprinting: bool,
}

fn parse_args() -> (CatOptions, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let mut options = CatOptions {
        number: false,
        number_nonblank: false,
        squeeze_blank: false,
        show_ends: false,
        show_tabs: false,
        show_nonprinting: false,
    };
    let mut files = Vec::new();

    let mut skip_next = false;
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }

        match arg.as_str() {
            "-b" | "--number-nonblank" => {
                options.number = true;
                options.number_nonblank = true;
            }
            "-e" => {
                options.show_ends = true;
                options.show_nonprinting = true;
            }
            "-n" | "--number" => {
                options.number = true;
            }
            "-s" | "--squeeze-blank" => {
                options.squeeze_blank = true;
            }
            "-t" => {
                options.show_tabs = true;
                options.show_nonprinting = true;
            }
            "-T" | "--show-tabs" => {
                options.show_tabs = true;
            }
            "-E" | "--show-ends" => {
                options.show_ends = true;
            }
            "-A" | "--show-all" => {
                options.show_nonprinting = true;
                options.show_ends = true;
                options.show_tabs = true;
            }
            "-v" | "--show-nonprinting" => {
                options.show_nonprinting = true;
            }
            "-u" => {
                // Ignored
            }
            "-h" | "--help" => {
                usage();
            }
            _ if arg.starts_with('-') => {
                eprintln!("Unknown option: {}", arg);
                usage();
            }
            _ => {
                files.push(arg.clone());
            }
        }
    }

    if files.is_empty() {
        files.push("-".to_string()); // Default to stdin
    }

    (options, files)
}

fn main() {
    let (options, files) = parse_args();
    let mut line_num = 0;

    let stdout = io::stdout();
    let mut writer = stdout.lock();

    for file in files {
        if let Err(err) = cat_file(&file, &options, &mut line_num, &mut writer) {
            eprintln!("Error reading file {}: {}", file, err);
            process::exit(1);
        }
    }
}
