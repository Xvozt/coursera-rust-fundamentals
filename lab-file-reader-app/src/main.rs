use std::fs::File;
use std::io::{BufRead, BufReader};

fn help() {
    println!("usage: pass the filepath");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!("Pass some filepath");
        }
        2 => match args[1].parse::<String>() {
            Ok(filepath) => {
                let file = File::open(filepath);
                let file = match file {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        std::io::ErrorKind::NotFound => {
                            panic!("File not found: {}", error)
                        }
                        _ => {
                            panic!("Error opening file: {}", error)
                        }
                    },
                };

                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(line) => println!("{}", line),
                        Err(error) => {
                            panic!("Error reading line: {}", error)
                        }
                    }
                }
            }
            Err(error) => {
                panic!("Error parsing filepath: {}", error);
            }
        },

        _ => {
            help();
        }
    }
}
