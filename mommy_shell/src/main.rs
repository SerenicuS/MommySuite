use std::env::set_current_dir;
use std::{env, fs};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;
use mommy_lib::responses;
use mommy_lib::constants;
use mommy_lib::shell_commands;
const SEPARATOR: &str = "----------------------------------------------------------------";


/*
    TODO:
        1. Handles many commands/arguments using the shell.

 */

const SHELL_BASIC_COMMANDS: &str = r#"
    You are too greedy.
    ---------------
     1. tellme                      ->    List Commands
     2. mayileave                   ->    Exit the Terminal
     3. iamhere                     ->    Locate current Directory
     4. mommy?                      ->    List Files in current Directory
     5. walkwithme <filename>       ->    Move to another Directory
     6. goback                      ->    Return to Previous Directory
     7. canihave <filename>         ->    Create File
     8. takethe <filename>          ->    Delete File
     9. openthis <filename>         ->    Open the File
    10. readthis <filename>         ->    Read the File's contents
    11. doxxme                      ->    Windows Ip Configuration
    12. callmeplease <ip/dns>       ->    Ping device
    13. runthis <filename>          ->    Run File
    ---------------
    "#;


const SHELL_ADVANCE_COMMANDS: &str = r#"
    You are too greedy.
    ---------------
     1. startcoding                 ->    Enter lite_IDE
    ---------------
    "#;




fn main() {
    let root_dir = env::current_dir().expect(&responses::MommyShellError::RootDirError.to_string());

    println!("{}", responses::MommyUI::WelcomeTitle);
    println!("{}", responses::MommyUI::WelcomeSubtitle);
    println!("{}", responses::MommyUI::WelcomePrompt);

    loop{

        let mut input = String::new();// lineBuffer
        println!();
        io::stdin().read_line(&mut input).expect(&responses::MommyUI::ExitMessage.to_string());

        match input.trim(){
            "Y" => shell_start_default(input, &root_dir),
            "T" => shell_skip_default(&root_dir),
            _ => std::process::exit(0),
        }
    }

}

fn shell_skip_default(root_dir: &std::path::PathBuf){
    shell_move_directory("sandbox", root_dir);
    run_mommy_lang("discipline-update-test.mommy");
}


fn shell_start_default(mut input: String, root_dir: &std::path::PathBuf) { // Added root_dir
    println!("{}", responses::MommyUI::GenericObedience);

    loop {
        input.clear();
        print!("{}", constants::SHELL_LINE_INDICATOR);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect(&responses::MommyUI::ExitMessage.to_string());



        println!("{}", SEPARATOR);


        shell_attempt_command(&input, root_dir);
    }
}

fn shell_open_file(file_name: &str){
    match Command::new("cmd").args(&["/C", "start", file_name]).output(){
        Ok(_) => println!("{}", responses::MommyShellOk::FileOpened),
        Err(_) => println!("{}", responses::MommyShellError::FileNotFound),
    }
}

fn shell_read_file(file_name: &str){
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("{}", responses::MommyShellError::CannotReadFile);
            return;
        }
    };

    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => println!("{}. {}", index + constants::SHELL_LINE_INCREMENTOR, l),
            Err(_) => break,
        }
    }
}

fn shell_return_to_prev_directory(root_dir: &std::path::PathBuf) {
    let current_dir = env::current_dir().unwrap();

    if current_dir.canonicalize().unwrap() == root_dir.canonicalize().unwrap() {
        println!("{}", responses::MommyShellError::RootDirectoryLocked);
    } else {
        match set_current_dir(constants::SHELL_PREVIOUS_DIRECTORY_KEYWORD) {
            Ok(_) => println!("{}", responses::MommyShellOk::DirectoryChanged),
            Err(_) => println!("{}", responses::MommyShellError::GeneralInvalid),
        }
    }
}


fn check_args_len(args: &Vec<&str>) -> bool {
        args.len() > 1
    }

fn shell_create_file(file_name: &str) {
    match File::create(file_name) {
        Ok(_) => println!("{}", responses::MommyShellOk::FileCreated),
        Err(_) => println!("{}", responses::MommyShellError::CannotCreateFile)
    }
}

fn shell_delete_file(file_name: &str) {
    match fs::remove_file(file_name) {
        Ok(_) => println!("{}", responses::MommyShellOk::FileDeleted),
        Err(_) => println!("{}", responses::MommyShellError::CannotDeleteFile),
    }
}
fn shell_list_files_in_directory() {
    let files = fs::read_dir(constants::SHELL_CURRENT_DIRECTORY_KEYWORD).expect(&responses::MommyShellError::CannotListFiles.to_string());

    for entry in files {
        let entry = entry.expect(&responses::MommyShellError::CannotListFiles.to_string());
        println!("{}", entry.path().display());
    }
}
fn shell_get_directory() {
    let dir = env::current_dir().expect(&responses::MommyShellError::DirectoryNotFound.to_string());
    println!("{}", dir.display());

}

fn shell_get_directory_return() -> String {
    let dir = env::current_dir().expect(&responses::MommyShellError::DirectoryNotFound.to_string());

    dir.display().to_string()
}
fn shell_print_basic_help() {
    println!("{}", SHELL_BASIC_COMMANDS);
}

fn shell_print_advance_help() {
    println!("{}", SHELL_ADVANCE_COMMANDS);
}

fn shell_move_directory(path: &str, root_dir: &std::path::PathBuf) {
    let current_dir = env::current_dir().unwrap();
    let target_path = current_dir.join(path);

    match target_path.canonicalize() {
        Ok(canonical_target) => {
            if canonical_target.starts_with(root_dir.canonicalize().unwrap()) {
                if set_current_dir(&canonical_target).is_ok() {
                    let raw_path = shell_get_directory_return();
                    let display_path = raw_path.replace(constants::WINDOWS_EXTENDED_LENGTH_PATH_PREFIX, constants::SHELL_EMPTY_STRING);
                    println!("Moved Inside: {}", display_path);
                }
            } else {
                println!("{}", responses::MommyShellError::RootDirectoryLocked);
            }
        }
        Err(_) => println!("{}", responses::MommyShellError::DirectoryNotFound),
    }
}



fn shell_attempt_command(input: &str, root_dir: &std::path::PathBuf) {
    let clean_input = input.trim();
    let args: Vec<&str> = clean_input.split_whitespace().collect();

    if args.is_empty() {
        println!("{}", responses::MommyShellError::IncompleteArgs);
        return;
    }

    let first_args = shell_commands::MommyShellCommands::from_str(args[constants::INDEX_DEFAULT_STARTING_COMMAND_ARGS]);

    match first_args { // 0
        //1 Args
        shell_commands::MommyShellCommands::ShellHelp => shell_print_basic_help(),
        shell_commands::MommyShellCommands::ShellHelpAdvanced => shell_print_advance_help(),
        shell_commands::MommyShellCommands::ShellExit => std::process::exit(0),
        shell_commands::MommyShellCommands::ShellCurrentDirectory => shell_get_directory(),
        shell_commands::MommyShellCommands::ShellListFilesCurrentDirectory => shell_list_files_in_directory(),
        shell_commands::MommyShellCommands::ShellShowIPConfig => shell_windows_call("ipconfig"),
        shell_commands::MommyShellCommands::ShellReturnToPrevDirectory => shell_return_to_prev_directory(root_dir),


        //Advanced
        shell_commands::MommyShellCommands::ShellStartCoding => shell_prepare_coding(),

        // 2 Args
        shell_commands::MommyShellCommands::ShellChangeDirectory if check_args_len(&args) => shell_move_directory(args[1], root_dir),
        shell_commands::MommyShellCommands::ShellCreateFile if check_args_len(&args) => shell_create_file(args[1]),
        shell_commands::MommyShellCommands::ShellDeleteFile if check_args_len(&args) => shell_delete_file(args[1]),
        shell_commands::MommyShellCommands::ShellOpenFile if check_args_len(&args) => shell_open_file(args[1]),
        shell_commands::MommyShellCommands::ShellRunFile if check_args_len(&args) => shell_run_file(args[1]),
        shell_commands::MommyShellCommands::ShellReadFile if check_args_len(&args) => shell_read_file(args[1]),

        // Error
        shell_commands::MommyShellCommands::ShellUnknownCommand => println!("{}", responses::MommyShellError::GeneralInvalid),
        _ => println!("{}", responses::MommyShellError::GeneralInvalid),
    }
}


fn shell_run_file(filename: &str) {
    let extension = Path::new(filename).extension().and_then(|ext| ext.to_str()).
        unwrap_or("");

    match extension { //file type selection NOTE: it does not run without specifying the name
        "mommy" => {
            if let Ok(full_path) = fs::canonicalize(filename) {
                run_mommy_lang(&full_path.to_string_lossy());
            } else {
                run_mommy_lang(filename);
            }
        },
        "txt" => {
            simple_exec(constants::RUN_NOTEPAD, filename);
        },
        "py" => {
            simple_exec(constants::RUN_PYTHON, filename);
        },
        _ => {
            println!("{}", responses::MommyShellError::CannotOpenFile)
        }
    }
}

// For running non-mommy files
fn simple_exec(tool: &str, filename: &str) {
    println!("Opening {} with {}...", filename, tool);
    Command::new(tool)
        .arg(filename)
        .status()
        .expect("Failed to run the command");
}

fn shell_prepare_coding() {
    let mut input = String::new();
    println!("{}", SEPARATOR);
    println!("{}", responses::MommyUI::PrepareCoding);
    println!("{}", responses::MommyUI::WelcomePrompt);
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect(&responses::MommyUI::ExitMessage.to_string());


    match input.trim() {
        "Y" => shell_start_coding(),
        _ => {
            println!("{}", responses::MommyUI::RefuseCoding);
            return;
        },
    }
}

fn shell_start_coding() {
    println!("{}", SEPARATOR);
    println!("{}", responses::MommyUI::StartCoding);
    println!("{}", SEPARATOR);

    let mut lite_ide = String::new();
    let mut line_count = 1;

    loop { // Writing process
        print!("{}. ", line_count);

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect(&responses::MommyShellError::CannotCreateFile.to_string());

        if input.trim() == constants::SHELL_IDE_SAVE_FILE_KEYWORD {
            break;
        } else if input.trim() == constants:: SHELL_IDE_EXIT_KEYWORD {
            return;
        } else if input.trim() == constants::SHELL_IDE_CLEAR_KEYWORD {
            lite_ide.clear();
            line_count = 1;
            println!("{}", responses::MommyUI::RestartCLI);
            continue
        }
        lite_ide.push_str(&input);

        line_count += 1;
    }

    shell_save_coding(&lite_ide);
}

fn shell_save_coding(lite_ide: &str) {
    println!("{}", SEPARATOR);
    println!("{}", responses::MommyLangStatus::RenameFile);

    let mut input_name = String::new();
    io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to read input");

    let clean_name = input_name.trim();

    let final_filename = validate_file(&clean_name);

    // Without this, fs::write crashes on a fresh install.
    let sandbox_dir = constants::IDE_OUTPUT_DIRECTORY;
    if !Path::new(sandbox_dir).exists() {
        if let Err(_) = fs::create_dir_all(sandbox_dir) {
            println!("Mommy Error: I tried to build the sandbox, but the OS said no.");
            return;
        }
    }

    let full_path = format!("{}/{}", sandbox_dir, final_filename);

    // Write and Run
    match fs::write(&full_path, lite_ide) {
        Ok(_) => {
            println!("{}", responses::MommyShellOk::FileCreated);
            shell_instant_run_mommy_file(&full_path);
        },
        Err(_) => println!("{}", responses::MommyShellError::CannotCreateFile),
    }
}

fn shell_instant_run_mommy_file(full_path: &str) {
    println!("{}", SEPARATOR);
    println!("{}", responses::MommyLangStatus::PrepareRun);
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();


    if ans.trim().eq_ignore_ascii_case("Y") { // Run it immediately
        run_mommy_lang(&full_path);
    } else {
        println!("{}", responses::MommyLangStatus::SaveOnly);
        return
    }
}

fn validate_file(clean_name: &str) -> String {
    if clean_name.ends_with(constants::EXTENSION_SOURCE) {
        clean_name.to_string()
    } else {
        format!("{}{}", clean_name, constants::EXTENSION_SOURCE)
    }
}

fn run_mommy_lang(filename: &str) {
    println!("{}", SEPARATOR);
    println!("{}", responses::MommyLangStatus::CheckingFile);

    let absolute_path = fs::canonicalize(filename)
        .unwrap_or_else(|_| std::path::PathBuf::from(filename));

    let clean_path = absolute_path.to_string_lossy().replace(
        constants::WINDOWS_EXTENDED_LENGTH_PATH_PREFIX, constants::SHELL_EMPTY_STRING);

    let (cmd, args) = if cfg!(debug_assertions) {
        ("cargo".to_string(), vec!["run".into(), "-p".into(), "mommy_lang".into(), "--".into(), clean_path]) // Do not touch this to be a const for now.
    } else {
        let mut path = env::current_exe().expect("Unable to get current process path");
        path.pop();

        if cfg!(target_os = "windows") {
            path.push("mommy_lang.exe");
        } else {
            path.push("mommy_lang");
        }

        if !path.exists() {
            println!("Mommy Error: I cannot find 'mommy_lang.exe'.");
            return;
        }

        (path.to_string_lossy().to_string(), vec![clean_path])
    };

    let status_result = Command::new(cmd)
        .args(&args)
        .status();

    println!("{}", SEPARATOR);

    match status_result {
        Ok(status) if status.success() => println!("{}", responses::MommyLangStatus::ResultOk),
        Err(_) => println!("{}", responses::MommyLangStatus::ResultError),
        _ => println!("{}", responses::MommyLangStatus::ResultError),
    }
}



/*

    WINDOWS COMMANDS
 */

fn shell_windows_call(windows_command: &str) {
    match windows_command {
        "ipconfig" => windows_command_console_output(windows_command),
        _ => println!("{}", responses::MommyShellError::ExternalIPConfigCallFail),
    }
}

fn windows_command_console_output(var: &str) {
    match Command::new(var).output() {
        Ok(output) => {
            let console_output = String::from_utf8_lossy(&output.stdout);
            println!("{}", console_output);
        }
        Err(_) => println!("{}", responses::MommyShellError::ExternalCommandFailed),
    }
}







