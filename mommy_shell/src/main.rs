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

use crate::file_ops::{shell_create_file, shell_delete_file, shell_open_file, shell_read_file, shell_rename_file};
use crate::dir_ops::{
    shell_create_dir,
    shell_delete_dir,
    shell_get_directory,
    shell_list_files_in_directory,
    shell_move_directory,
    shell_return_to_prev_directory,
};
use crate::config_ops::{shell_change_code_dir, shell_change_username};
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
            shell_change_username(&user_name, mommy_settings);
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

    if mommy_settings.username_does_not_exist(){

        shell_change_username(constants::SHELL_DF_USER, mommy_settings);
    }


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
    let args: Vec<String> = lex_shell_input(input.trim());

    if args.is_empty() {
        print_line(responses::MommyShellError::IncompleteArgs);
        return;
    }

    let command_str = &args[constants::IDX_STARTING_COMMAND];
    let first_args = shell_commands::MommyShellCommands::from_str(command_str);

    let passed_args = &args[1..];

    match (first_args, passed_args) {

        // ==========================================
        // COMMANDS THAT REQUIRE EXACTLY 0 ARGUMENTS
        // ==========================================
        (shell_commands::MommyShellCommands::ShellHelp, []) => shell_print_basic_help(),
        (shell_commands::MommyShellCommands::ShellHelpAdvanced, []) => shell_print_advance_help(),
        (shell_commands::MommyShellCommands::ShellExit, []) => std::process::exit(0),
        (shell_commands::MommyShellCommands::ShellCurrentDirectory, []) => shell_get_directory(),
        (shell_commands::MommyShellCommands::ShellListFilesCurrentDirectory, []) => shell_list_files_in_directory(),
        (shell_commands::MommyShellCommands::ShellShowIPConfig, []) => shell_windows_call("ipconfig"),
        (shell_commands::MommyShellCommands::ShellReturnToPrevDirectory, []) => shell_return_to_prev_directory(root_dir),
        (shell_commands::MommyShellCommands::ShellClear, []) => shell_clear(),
        (shell_commands::MommyShellCommands::ShellStartCoding, []) => shell_prepare_coding(root_dir, mommy_settings),

        // ==========================================
        // COMMANDS THAT REQUIRE EXACTLY 1 ARGUMENT
        // ==========================================
        (shell_commands::MommyShellCommands::ShellChangeDirectory, [arg]) => shell_move_directory(arg, root_dir),
        (shell_commands::MommyShellCommands::ShellCreateFile, [arg]) => shell_create_file(arg),
        (shell_commands::MommyShellCommands::ShellDeleteFile, [arg]) => shell_delete_file(arg),
        (shell_commands::MommyShellCommands::ShellOpenFile, [arg]) => shell_open_file(arg, root_dir),
        (shell_commands::MommyShellCommands::ShellRunFile, [arg]) => shell_run_file(arg, &mommy_settings.output_directory),
        (shell_commands::MommyShellCommands::ShellReadFile, [arg]) => shell_read_file(arg),
        (shell_commands::MommyShellCommands::ShellCreateDir, [arg]) => shell_create_dir(arg),
        (shell_commands::MommyShellCommands::ShellDeleteDir, [arg]) => shell_delete_dir(arg),
        (shell_commands::MommyShellCommands::ShellChangeCodeDir, [arg]) => shell_change_code_dir(arg, mommy_settings),


        // ==========================================
        // COMMANDS THAT REQUIRE EXACTLY 2 ARGUMENT
        // ==========================================
        (shell_commands::MommyShellCommands::ShellRenameFile, [old_file_name, new_file_name]) => shell_rename_file(old_file_name, new_file_name),


        // ==========================================
        // ERROR HANDLING / CATCH-ALLS
        // ==========================================
        (shell_commands::MommyShellCommands::ShellUnknownCommand, _) => print_line(responses::MommyShellError::GeneralInvalid),

        _ => {
            print_line(responses::MommyShellError::GeneralInvalid)
        },
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

    fn lex_shell_input(input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_quotes = false;

        for c in input.chars() {
            match c {
                '"' => in_quotes = !in_quotes,

                ' ' | '\t' if !in_quotes => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }

                _ => current_token.push(c),
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        tokens
    }
