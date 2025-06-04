use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let file = File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied: {}", error)
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

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("to_write.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("Could not create file: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied when creating file: {}", error)
            }
            std::io::ErrorKind::AlreadyExists => {
                panic!("File already exists and cannot be overwritten: {}", error)
            }
            _ => {
                panic!("Error creating file: {}", error)
            }
        },
    };

    let mut writer = BufWriter::new(file);
    let write_result = writer.write_all(b"Hello, world!\n");
    match write_result {
        Ok(_) => {}
        Err(error) => match error.kind() {
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied: {}", error)
            }
            std::io::ErrorKind::WriteZero => {
                panic!("Failed to write any bytes: {}", error)
            }
            std::io::ErrorKind::Interrupted => {
                panic!("Interrupted while writing to file: {}", error)
            }
            _ => {
                panic!("Error writing to file: {}", error)
            }
        },
    }
}
