use std::env;

use mommy_lib::constants;
use mommy_lib::responses;
use mommy_lib::shell_format::print_line;

pub fn shell_move_directory(path: &str, root_dir: &std::path::PathBuf) {
    let current_dir = env::current_dir().unwrap();
    let target_path = current_dir.join(path);

    match target_path.canonicalize() {
        Ok(canonical_target) => {
            if canonical_target.starts_with(root_dir.canonicalize().unwrap()) {
                if env::set_current_dir(&canonical_target).is_ok() {
                    let raw_path = shell_get_directory_return();
                    let display_path = raw_path.replace(constants::SHELL_PATH_PREFIX, constants::SHELL_EMPTY);
                    print_line(format!("Moved Inside: {}", display_path));
                }
            } else {
                print_line(responses::MommyShellError::RootDirectoryLocked);
            }
        }
        Err(_) => print_line(responses::MommyShellError::DirectoryNotFound),
    }
}

pub fn shell_return_to_prev_directory(root_dir: &std::path::PathBuf) {
    let current_dir = env::current_dir().unwrap();

    if current_dir.canonicalize().unwrap() == root_dir.canonicalize().unwrap() {
        print_line(responses::MommyShellError::RootDirectoryLocked);
    } else {
        match env::set_current_dir(constants::SHELL_DIR_PREV) {
            Ok(_) => print_line(responses::MommyShellOk::DirectoryChanged),
            Err(_) => print_line(responses::MommyShellError::GeneralInvalid),
        }
    }
}

pub fn shell_create_dir(dir_name: &str) {
    match std::fs::create_dir(dir_name) {
        Ok(_) => print_line(responses::MommyShellOk::DirectoryCreated),
        Err(_) => print_line(responses::MommyShellError::DirectoryNotFound),
    }
}

pub fn shell_delete_dir(dir_name: &str) {
    match std::fs::remove_dir(dir_name) {
        Ok(_) => print_line(responses::MommyShellOk::DirectoryDeleted),
        Err(_) => print_line(responses::MommyShellError::DirectoryNotFound),
    }
}

pub fn shell_get_directory() {
    let dir = env::current_dir().expect(&responses::MommyShellError::DirectoryNotFound.to_string());
    println!("{}", dir.display());
}

pub fn shell_get_directory_return() -> String {
    let dir = env::current_dir().expect(&responses::MommyShellError::DirectoryNotFound.to_string());
    dir.display().to_string()
}

pub fn shell_list_files_in_directory() {
    let files = std::fs::read_dir(constants::SHELL_DIR_CURR)
        .expect(&responses::MommyShellError::CannotListFiles.to_string());

    for entry in files {
        let entry = entry.expect(&responses::MommyShellError::CannotListFiles.to_string());
        println!("{}", entry.path().display());
    }
}

