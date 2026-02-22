use std::process::Command;
use std::path::PathBuf;
use mommy_lib::config;
use mommy_lib::responses;
use mommy_lib::shell_format::{print_line, print_wrapper, read_prompted_line};
use mommy_lib::constants;

use crate::file_validation;

pub fn shell_prepare_coding(root_dir: &PathBuf, mommy_settings: &mut config::MommySettings) {
    print_wrapper([
        responses::MommyUI::PrepareCoding.to_string(),
        responses::MommyUI::WelcomePrompt.to_string(),
    ]);

    let input = read_prompted_line(constants::SPACE_PROMPT);

    match input.trim() {
        constants::LETTER_SHELL_YES => shell_start_coding(root_dir, mommy_settings),
        _ => {
            print_line(responses::MommyUI::RefuseCoding);
        }
    }
}

pub fn shell_start_coding(root_dir: &PathBuf, mommy_settings: &mut config::MommySettings) {
    let editor_path = root_dir.join("mommy_editor").join("mommy_editor.exe");

    if !file_validation::does_file_exist(&editor_path) {
        print_line(responses::MommyShellError::CannotFindTextEditor);
        return;
    }

    match Command::new(&editor_path)
        .arg(&mommy_settings.output_directory)
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
