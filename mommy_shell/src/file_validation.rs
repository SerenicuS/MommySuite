use std::env;
use std::path::PathBuf;
use mommy_lib::{constants, responses};

pub fn shell_get_directory_return() -> String {
    let dir = env::current_dir()
        .expect(&responses::MommyShellError::DirectoryNotFound.to_string());
    dir.display().to_string()  // Convert PathBuf to String
}


pub fn does_file_exist(file: &PathBuf) -> bool{
     if file.exists() {
         return true
     }

     false
}

pub fn clean_path_with_win_prefix(dir: &str) -> String{
    if dir.starts_with(constants::SHELL_PATH_PREFIX) {
        return dir[4..].to_string()
    };

    dir.to_string()
}


