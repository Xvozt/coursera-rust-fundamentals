//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use documentaion_lab::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read input line".

use std::io::{BufRead, BufReader};

pub mod colors;
pub mod config;

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use documentaion_lab::read_stdin;
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::_read_stdin;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_read_input_with_newline() {
        let input = "Hello, world!\n";
        let expected_output = "Hello, world!";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_input_empty() {
        let input = "";
        let expected_output = "";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_input_whitespace_at_start() {
        let input = " Hello world\n";
        let expected_output = "Hello world";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    #[should_panic(expected = "Failed to read input line")]
    fn test_read_with_error_reader() {
        struct ErrorReader;

        impl std::io::Read for ErrorReader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "My Simulated Error",
                ))
            }
        }

        impl std::io::BufRead for ErrorReader {
            fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "My Simulated Error",
                ))
            }

            fn consume(&mut self, amt: usize) {
                // No-op
            }
        }
        let input = "Hello world ";
        let mut reader = ErrorReader;
        _read_stdin(&mut reader);
    }
}
