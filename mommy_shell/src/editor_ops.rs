use std::fs;
use std::io::{self, Write};
use std::path::Path;

use mommy_lib::config;
use mommy_lib::constants;
use mommy_lib::responses;
use mommy_lib::shell_format::{print_line, print_wrapper, read_prompted_line};

use crate::exec_ops::run_mommy_lang;

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
    print_wrapper([
        responses::MommyUI::StartCoding.to_string(),
    ]);

    let mut lite_ide = String::new();
    let mut line_count = 1;

    loop {
        print!("{}. ", line_count);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(&responses::MommyShellError::CannotCreateFile.to_string());

        if input.trim() == constants::SHELL_CMD_SAVE {
            break;
        } else if input.trim() == constants::SHELL_CMD_EXIT {
            return;
        } else if input.trim() == constants::SHELL_CMD_CLEAR {
            lite_ide.clear();
            line_count = 1;
            print_line(responses::MommyUI::RestartCLI);
            continue;
        }
        lite_ide.push_str(&input);
        line_count += 1;
    }

    shell_save_coding(&lite_ide, mommy_settings);
}

pub fn shell_save_coding(lite_ide: &str, mommy_settings: &config::MommySettings) {
    print_wrapper([
        responses::MommyLangStatus::RenameFile.to_string(),
    ]);

    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect("Failed to read input");
    let clean_name = input_name.trim();
    let final_filename = validate_file(clean_name);

    let target_dir_path = Path::new(&mommy_settings.output_directory);

    if !target_dir_path.exists() {
        if let Err(_) = fs::create_dir_all(target_dir_path) {
            print_line(format!("Mommy Error: I tried to go to '{}', but I couldn't create it.", mommy_settings.output_directory));
            return;
        }
    }

    let full_path = target_dir_path.join(final_filename);

    match fs::write(&full_path, lite_ide) {
        Ok(_) => {
            print_line(responses::MommyShellOk::FileCreated);
            shell_instant_run_mommy_file(full_path.to_str().unwrap());
        }
        Err(_) => print_line(responses::MommyShellError::CannotCreateFile),
    }
}

pub fn shell_instant_run_mommy_file(full_path: &str) {
    print_wrapper([
        responses::MommyLangStatus::PrepareRun.to_string(),
    ]);

    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();

    if ans.trim().eq_ignore_ascii_case(constants::LETTER_SHELL_YES) {
        run_mommy_lang(full_path);
    } else {
        print_line(responses::MommyLangStatus::SaveOnly);
    }
}

fn validate_file(clean_name: &str) -> String {
    if clean_name.ends_with(constants::EXT_SOURCE) {
        clean_name.to_string()
    } else {
        format!("{}{}", clean_name, constants::EXT_SOURCE)
    }
}

