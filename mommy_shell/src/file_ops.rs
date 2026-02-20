use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
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

pub fn shell_open_file(file_name: &str, root_dir: &PathBuf) {
    let dir = shell_get_directory_return();

    // Clean up Windows UNC prefix
    let clean_dir = if dir.starts_with("\\\\?\\") {
        dir[4..].to_string()
    } else {
        dir
    };

    // Build absolute path
    let mut full_path = PathBuf::from(&clean_dir);
    full_path.push(file_name);

    // Check if file exists
    if !full_path.exists() {
        print_line(responses::MommyShellError::FileNotFound);
        return;
    }

    // Get editor path from root_dir
    let editor_path = root_dir.join("mommy_editor").join("mommy_editor.exe");
    
    // Verify editor exists before attempting to launch
    if !editor_path.exists() {
        eprintln!("ERROR: Editor not found at: {}", editor_path.display());
        print_line(responses::MommyShellError::FileNotFound);
        return;
    }

    match Command::new(&editor_path)
        .arg(full_path.to_string_lossy().to_string())
        .status()
    {
        Ok(status) => {
            if status.success() {
                print_line(responses::MommyShellOk::FileOpened);
            } else {
                eprintln!("WARNING: Editor exited with error code: {:?}", status.code());
                print_line(responses::MommyShellError::FileNotFound);
            }
        },
        Err(e) => {
            eprintln!("ERROR: Failed to launch editor: {}", e);
            print_line(responses::MommyShellError::FileNotFound);
        }
    }
}


pub fn shell_get_directory_return() -> String {
    let dir = env::current_dir()
        .expect(&responses::MommyShellError::DirectoryNotFound.to_string());
    dir.display().to_string()  // Convert PathBuf to String
}

