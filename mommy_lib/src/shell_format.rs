//!
//! This is the shell format crate of mommyshell.
//!
//!
//!
//!
//!
//!

use std::io::{self, Write};
use crate::constants;

// Render a narrative block with separators and indentation.
pub fn print_wrapper<I>(lines: I)
where
    I: IntoIterator<Item = String>,
{
    println!("{}", constants::SEPARATOR_THICK);
    for line in lines {
        println!("\x1B[31m{}{}\x1B[0m", constants::INDENT_OUTPUT, line);
    }
    println!("{}", constants::SEPARATOR_THICK);
}

// Convenience for single-line narrative output.
pub fn print_line<T: std::fmt::Display>(line: T) {
    print_wrapper([line.to_string()]);
}

// Stderr variant for errors and warnings.
pub fn eprint_line<T: std::fmt::Display>(line: T) {
    eprintln!("{}{}", constants::INDENT_OUTPUT, line);
}

// Prompt without newline so user input stays on the same line.
pub fn print_prompt(user: &str, prompt: &str) {
    print!("{}{}", user, prompt);
    io::stdout().flush().unwrap();
}

// Prompt + stdin read with default error handling.
pub fn read_prompted_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input
}

// Prompt + stdin read with caller-provided error message.
pub fn read_prompted_line_with_error(prompt: &str, error_message: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect(error_message);
    input
}

