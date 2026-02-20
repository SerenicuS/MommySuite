use std::process::Command;
use mommy_lib::config;
use mommy_lib::responses;
use mommy_lib::shell_format::{print_line, print_wrapper, read_prompted_line};
use mommy_lib::constants;
use crate::editor_common::get_editor_path;

pub fn shell_prepare_coding(mommy_settings: &mut config::MommySettings) {
    print_wrapper([
        responses::MommyUI::PrepareCoding.to_string(),
        responses::MommyUI::WelcomePrompt.to_string(),
    ]);

    let input = read_prompted_line(constants::SPACE_PROMPT);

    match input.trim() {
        constants::LETTER_SHELL_YES => shell_start_coding(mommy_settings),
        _ => {
            print_line(responses::MommyUI::RefuseCoding);
        }
    }
}

pub fn shell_start_coding(mommy_settings: &mut config::MommySettings) {
    let editor_path = get_editor_path();

    // Verify editor exists before attempting to launch
    if !editor_path.exists() {
        eprintln!("ERROR: Editor not found at: {}", editor_path.display());
        print_line(responses::MommyShellError::FileNotFound);
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
