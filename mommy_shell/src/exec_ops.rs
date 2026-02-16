use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use mommy_lib::constants;
use mommy_lib::responses;
use mommy_lib::shell_format::{print_line, print_wrapper};

pub fn run_mommy_lang(filename: &str) {
    print_wrapper([
        responses::MommyLangStatus::CheckingFile.to_string(),
    ]);

    let absolute_path = fs::canonicalize(filename)
        .unwrap_or_else(|_| std::path::PathBuf::from(filename));

    let clean_path = absolute_path
        .to_string_lossy()
        .replace(constants::SHELL_PATH_PREFIX, constants::SHELL_EMPTY);

    let (cmd, args) = if cfg!(debug_assertions) {
        (
            "cargo".to_string(),
            vec!["run".into(), "-p".into(), "mommy_lang".into(), "--".into(), clean_path],
        )
    } else {
        let mut path = env::current_exe().expect("Unable to get current process path");
        path.pop();

        if cfg!(target_os = "windows") {
            path.push("mommy_lang.exe");
        } else {
            path.push("mommy_lang");
        }

        if !path.exists() {
            print_line("Mommy Error: I cannot find 'mommy_lang.exe'.");
            return;
        }

        (path.to_string_lossy().to_string(), vec![clean_path])
    };

    let status_result = Command::new(cmd).args(&args).status();

    println!("{}", constants::SEPARATOR);

    match status_result {
        Ok(status) if status.success() => print_line(responses::MommyLangStatus::ResultOk),
        Err(_) => print_line(responses::MommyLangStatus::ResultError),
        _ => print_line(responses::MommyLangStatus::ResultError),
    }
}

pub fn shell_run_file(file_name: &str, output_dir: &str) {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "mommy" => {
            let base_path = Path::new(output_dir);
            let target_path = base_path.join(file_name);

            if target_path.exists() {
                run_mommy_lang(target_path.to_str().unwrap());
            } else if Path::new(file_name).exists() {
                run_mommy_lang(file_name);
            } else {
                print_line(format!(
                    "Mommy Error: I cannot find '{}' in '{}' or the current folder.",
                    file_name, output_dir
                ));
            }
        }
        "txt" => {
            simple_exec(constants::CMD_RUN_NOTEPAD, file_name);
        }
        "py" => {
            simple_exec(constants::CMD_RUN_PYTHON, file_name);
        }
        _ => {
            print_line(responses::MommyShellError::CannotOpenFile);
        }
    }
}

pub fn simple_exec(tool: &str, filename: &str) {
    print_line(format!("Opening {} with {}...", filename, tool));
    Command::new(tool)
        .arg(filename)
        .status()
        .expect("Failed to run the command");
}

