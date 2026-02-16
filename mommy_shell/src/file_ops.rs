use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

use mommy_lib::responses;
use mommy_lib::constants;
use mommy_lib::shell_format::print_line;

pub fn shell_create_file(file_name: &str) {
    match File::create(file_name) {
        Ok(_) => print_line(responses::MommyShellOk::FileCreated),
        Err(_) => print_line(responses::MommyShellError::CannotCreateFile),
    }
}

pub fn shell_delete_file(file_name: &str) {
    match std::fs::remove_file(file_name) {
        Ok(_) => print_line(responses::MommyShellOk::FileDeleted),
        Err(_) => print_line(responses::MommyShellError::CannotDeleteFile),
    }
}

pub fn shell_read_file(file_name: &str) {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            print_line(responses::MommyShellError::CannotReadFile);
            return;
        }
    };

    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => println!("{}. {}", index + constants::SHELL_LINE_INC, l),
            Err(_) => break,
        }
    }
}

pub fn shell_open_file(file_name: &str) {
    match Command::new("cmd").args(&["/C", "start", file_name]).output() {
        Ok(_) => print_line(responses::MommyShellOk::FileOpened),
        Err(_) => print_line(responses::MommyShellError::FileNotFound),
    }
}

