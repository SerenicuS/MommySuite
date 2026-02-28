use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use mommy_lib::responses;
use mommy_lib::constants;
use mommy_lib::shell_format::print_line;

use crate::file_validation;

pub fn shell_create_file(file_name: &str) {
    match File::create(file_name) {
        Ok(_) => print_line(responses::MommyShellOk::FileCreated),
        Err(_) => print_line(responses::MommyShellError::CannotCreateFile),
    }
}

pub fn shell_delete_file(file_name: &str) {

    if file_name.starts_with(constants::MOMMY_DIR_PREFIX){
        print_line(responses::MommyShellError::CannotDeleteFile);
        return
    }

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

pub fn shell_rename_file(old_file_name: &str, new_file_name: &str) {

    let cur_path = env::current_dir();

    let full_curr_path = cur_path.unwrap().join(old_file_name);

    if !full_curr_path.exists() {
        print_line(responses::MommyShellError::FileNotFound);
        return
    }

    if old_file_name.starts_with(constants::MOMMY_DIR_PREFIX){
        print_line(responses::MommyShellError::NotAllowedToRenameFile);
        return
    }

    match fs::rename(old_file_name, new_file_name) {
        Ok(_) => print_line(responses::MommyShellOk::FileRenamed),
        Err(_) => print_line(responses::MommyShellError::CannotRenameFile),
    }

}

pub fn shell_open_file(file_name: &str, root_dir: &PathBuf) {
    let dir = file_validation::shell_get_directory_return();

    let clean_dir: String = {
        file_validation::clean_path_with_win_prefix(&dir)
    };

    let mut full_path = PathBuf::from(&clean_dir);
    full_path.push(file_name);

    if !file_validation::does_file_exist(&full_path) {
        print_line(responses::MommyShellError::FileNotFound);
        return
    }

    let editor_path = if cfg!(debug_assertions) {
    // DEBUG PATH: Points to your actual source/project folder
    root_dir
        .join("mommy_editor")
        .join(constants::TXT_EDITOR_EXE)
    } else {
    // RELEASE PATH: Points to the final bundled location
         root_dir
        .join(constants::TXT_EDITOR_EXE)
    };

    if !file_validation::does_file_exist(&editor_path){
        print_line(responses::MommyShellError::CannotFindTextEditor);
        return
    }

    match Command::new(&editor_path)
        .arg(full_path.to_string_lossy().to_string())
        .status()
    {
        Ok(status) => {
            if status.success() {
                print_line(responses::MommyShellOk::FileOpened);
            } else {
                print_line(responses::MommyShellError::GeneralTextEditorError);
            }
        },
        Err(e) => {
            eprintln!("ERROR: Failed to launch editor: {}", e);
        }
    }
}