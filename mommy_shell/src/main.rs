mod file_ops;
mod dir_ops;
mod config_ops;
mod editor_ops;
mod exec_ops;
mod help_ops;
mod windows_ops;
mod file_validation;

use std::{env, io};
use std::io::Write;
use std::path::PathBuf;
use mommy_lib::responses;
use mommy_lib::constants;
use mommy_lib::shell_commands;
use mommy_lib::config::MommySettings;
use mommy_lib::shell_format::{print_wrapper, print_line, print_prompt, read_prompted_line_with_error};

use crate::file_ops::{shell_create_file, shell_delete_file, shell_open_file, shell_read_file};
use crate::dir_ops::{
    shell_create_dir,
    shell_delete_dir,
    shell_get_directory,
    shell_list_files_in_directory,
    shell_move_directory,
    shell_return_to_prev_directory,
};
use crate::config_ops::{shell_change_code_dir, shell_override_user};
use crate::editor_ops::shell_prepare_coding;
use crate::exec_ops::{shell_run_file};
use crate::help_ops::{shell_print_advance_help, shell_print_basic_help};
use crate::windows_ops::shell_windows_call;

fn main() {
    let root_dir_str = env::var("MOMMY_ROOT_DIR").expect("SECURITY VIOLATION: Shell launched outside of MommySuite OS.");
    let root_dir = PathBuf::from(root_dir_str);

    // 2. Load the config (we know it's safe because the OS already built it!)
    let mut mommy_settings = MommySettings::load(&root_dir);

    // 3. Begin the Interrogation
    shell_ask_user(&root_dir, &mut mommy_settings);
}

// ============================================================================
// SHELL INITIALIZATION & MAIN LOOP
// ============================================================================


fn shell_ask_user(root_dir: &PathBuf, mommy_settings: &mut MommySettings) {
    print_wrapper([
        responses::MommyUI::AskName.to_string(),
        responses::MommyUI::ConfirmName.to_string(),
    ]);

    let mut user_name = String::new();
    let mut anger_level = 0;
    let mut mind_wipe = false;

    loop {
        user_name.clear();
        user_name = read_prompted_line_with_error(
            constants::SPACE_PROMPT,
            &responses::MommyUI::ExitMessage.to_string(),
        );

        check_user_chance(&anger_level);

        if validate_user_input(&anger_level, &user_name) {
            mind_wipe = true;
            continue
        };

        if mind_wipe {
            shell_override_user(&user_name, mommy_settings);
            break
        };

        if check_default_user(&user_name, mommy_settings) {
            break;
        }

        anger_level += 1;
        println!("{}{}", constants::INDENT_OUTPUT, responses::MommyUI::RejectName);

    }

    // Main shell handles the welcome narrative.
    shell_start_default(&root_dir, mommy_settings);
}


fn check_default_user(user_name: &str, mommy_settings:  &MommySettings) -> bool{
    if user_name.trim() == mommy_settings.user_name || user_name.trim() == constants::SHELL_DF_USER{
        true
    }
    else{
        false
    }
}

fn check_user_chance(anger_level: &usize){
    if anger_level > &constants::VALERIA_ANGRY_METER_LIMIT{
        std::process::exit(0);
    };
}

fn validate_user_input(anger_level: &usize, pass: &str) -> bool{
    if anger_level >= &1 && pass.trim() == constants::SHELL_DF_PASS{
         print_wrapper([
                responses::MommyUI::MommyDoubt.to_string(),
            ]);
        true
    }
    else{
        false
    }
}


fn shell_start_default(root_dir: &PathBuf, mommy_settings: &mut MommySettings) {
    print_wrapper([
        responses::MommyUI::GenericObedience.to_string(),
        format!("{}, {}.", responses::MommyUI::MommyAcknowledge, mommy_settings.user_name),
    ]);

    let mut input = String::new();
    loop {
        input.clear();
        print_prompt(&mommy_settings.user_name, constants::SPACE_PROMPT);

        io::stdin()
            .read_line(&mut input)
            .expect(&responses::MommyUI::ExitMessage.to_string());


        shell_attempt_command(&input, root_dir, mommy_settings);

    }
}

fn shell_attempt_command(input: &str, root_dir: &PathBuf, mommy_settings: &mut MommySettings) {
    let clean_input = input.trim();
    let args: Vec<&str> = clean_input.split_whitespace().collect();

    if args.is_empty() {
        print_line(responses::MommyShellError::IncompleteArgs);
        return;
    }

    let first_args = shell_commands::MommyShellCommands::from_str(args[constants::IDX_STARTING_COMMAND]);

    match first_args {
        shell_commands::MommyShellCommands::ShellHelp => shell_print_basic_help(),
        shell_commands::MommyShellCommands::ShellHelpAdvanced => shell_print_advance_help(),
        shell_commands::MommyShellCommands::ShellExit => std::process::exit(0),
        shell_commands::MommyShellCommands::ShellCurrentDirectory => shell_get_directory(),
        shell_commands::MommyShellCommands::ShellListFilesCurrentDirectory => shell_list_files_in_directory(),
        shell_commands::MommyShellCommands::ShellShowIPConfig => shell_windows_call("ipconfig"),
        shell_commands::MommyShellCommands::ShellReturnToPrevDirectory => shell_return_to_prev_directory(root_dir),
        shell_commands::MommyShellCommands::ShellClear => shell_clear(),
        shell_commands::MommyShellCommands::ShellStartCoding => shell_prepare_coding(root_dir, mommy_settings),
        shell_commands::MommyShellCommands::ShellChangeDirectory if check_args_len(&args) => shell_move_directory(args[1], root_dir),
        shell_commands::MommyShellCommands::ShellCreateFile if check_args_len(&args) => shell_create_file(args[1]),
        shell_commands::MommyShellCommands::ShellDeleteFile if check_args_len(&args) => shell_delete_file(args[1]),
        shell_commands::MommyShellCommands::ShellOpenFile if check_args_len(&args) => shell_open_file(args[1], root_dir),
        shell_commands::MommyShellCommands::ShellRunFile if check_args_len(&args) => shell_run_file(args[1], &mommy_settings.output_directory),
        shell_commands::MommyShellCommands::ShellReadFile if check_args_len(&args) => shell_read_file(args[1]),
        shell_commands::MommyShellCommands::ShellCreateDir if check_args_len(&args) => shell_create_dir(args[1]),
        shell_commands::MommyShellCommands::ShellDeleteDir if check_args_len(&args) => shell_delete_dir(args[1]),
        shell_commands::MommyShellCommands::ShellChangeCodeDir if check_args_len(&args) => shell_change_code_dir(args[1], mommy_settings),
        shell_commands::MommyShellCommands::ShellUnknownCommand => print_line(responses::MommyShellError::GeneralInvalid),
        _ => print_line(responses::MommyShellError::GeneralInvalid),
    }
}

// ============================================================================
// HELP & UI
// ============================================================================

fn shell_clear() {
    print!("{}", responses::MommyUI::Clear);
    io::stdout().flush().unwrap();
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn check_args_len(args: &Vec<&str>) -> bool {
    args.len() > 1
}
