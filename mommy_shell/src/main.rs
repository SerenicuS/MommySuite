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
use mommy_lib::config;





fn main() {
    let root_dir = env::current_dir().expect(&responses::MommyShellError::RootDirError.to_string());
    let mut mommy_settings = config::MommySettings::load(&root_dir);

    println!("{}", responses::MommyUI::WelcomeTitle);
    println!("{}", responses::MommyUI::WelcomeSubtitle);
    println!("{}", responses::MommyUI::WelcomePrompt);

    loop{

        let mut input = String::new();// lineBuffer
        println!();
        io::stdin().read_line(&mut input).expect(&responses::MommyUI::ExitMessage.to_string());

        match input.trim(){
            "Y" => shell_start_default(input, &root_dir, &mut mommy_settings),
            "T" => shell_skip_default(&root_dir),
            _ => std::process::exit(0),
        }
    }

}

fn shell_skip_default(root_dir: &std::path::PathBuf){
    shell_move_directory("sandbox", root_dir);
    run_mommy_lang("discipline-update-test.mommy"); // test
}


fn shell_start_default(mut input: String, root_dir: &std::path::PathBuf, mommy_settings: &mut config::MommySettings) { // Added root_dir
    println!("{}", responses::MommyUI::GenericObedience);

    loop {
        input.clear();
        print!("{}", constants::SHELL_PROMPT);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect(&responses::MommyUI::ExitMessage.to_string());



        println!("{}", constants::SEPARATOR);


        shell_attempt_command(&input, root_dir, mommy_settings);
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
            Ok(l) => println!("{}. {}", index + constants::SHELL_LINE_INC, l),
            Err(_) => break,
        }
    }
}

fn shell_return_to_prev_directory(root_dir: &std::path::PathBuf) {
    let current_dir = env::current_dir().unwrap();

    if current_dir.canonicalize().unwrap() == root_dir.canonicalize().unwrap() {
        println!("{}", responses::MommyShellError::RootDirectoryLocked);
    } else {
        match set_current_dir(constants::SHELL_DIR_PREV) {
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
    let files = fs::read_dir(constants::SHELL_DIR_CURR).expect(&responses::MommyShellError::CannotListFiles.to_string());

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
    println!("{}", constants::SHELL_BASIC_COMMANDS);
}

fn shell_print_advance_help() {
    println!("{}", constants::SHELL_ADVANCE_COMMANDS);
}

fn shell_move_directory(path: &str, root_dir: &std::path::PathBuf) {
    let current_dir = env::current_dir().unwrap();
    let target_path = current_dir.join(path);

    match target_path.canonicalize() {
        Ok(canonical_target) => {
            if canonical_target.starts_with(root_dir.canonicalize().unwrap()) {
                if set_current_dir(&canonical_target).is_ok() {
                    let raw_path = shell_get_directory_return();
                    let display_path = raw_path.replace(constants::SHELL_PATH_PREFIX, constants::SHELL_EMPTY);
                    println!("Moved Inside: {}", display_path);
                }
            } else {
                println!("{}", responses::MommyShellError::RootDirectoryLocked);
            }
        }
        Err(_) => println!("{}", responses::MommyShellError::DirectoryNotFound),
    }
}



fn shell_attempt_command(input: &str, root_dir: &std::path::PathBuf, mommy_settings: &mut config::MommySettings) {
    let clean_input = input.trim();
    let args: Vec<&str> = clean_input.split_whitespace().collect();

    if args.is_empty() {
        println!("{}", responses::MommyShellError::IncompleteArgs);
        return;
    }

    let first_args = shell_commands::MommyShellCommands::from_str(args[constants::IDX_STARTING_COMMAND]);

    match first_args { // 0
        //1 Args
        shell_commands::MommyShellCommands::ShellHelp => shell_print_basic_help(),
        shell_commands::MommyShellCommands::ShellHelpAdvanced => shell_print_advance_help(),
        shell_commands::MommyShellCommands::ShellExit => std::process::exit(0),
        shell_commands::MommyShellCommands::ShellCurrentDirectory => shell_get_directory(),
        shell_commands::MommyShellCommands::ShellListFilesCurrentDirectory => shell_list_files_in_directory(),
        shell_commands::MommyShellCommands::ShellShowIPConfig => shell_windows_call("ipconfig"),
        shell_commands::MommyShellCommands::ShellReturnToPrevDirectory => shell_return_to_prev_directory(root_dir),
        shell_commands::MommyShellCommands::ShellClear => shell_clear(),


        //Advance
        shell_commands::MommyShellCommands::ShellStartCoding => shell_prepare_coding(mommy_settings),

        // 2 Args
        shell_commands::MommyShellCommands::ShellChangeDirectory if check_args_len(&args) => shell_move_directory(args[1], root_dir),
        shell_commands::MommyShellCommands::ShellCreateFile if check_args_len(&args) => shell_create_file(args[1]),
        shell_commands::MommyShellCommands::ShellDeleteFile if check_args_len(&args) => shell_delete_file(args[1]),
        shell_commands::MommyShellCommands::ShellOpenFile if check_args_len(&args) => shell_open_file(args[1]),
        shell_commands::MommyShellCommands::ShellRunFile if check_args_len(&args) => shell_run_file(args[1], &mommy_settings.output_directory),
        shell_commands::MommyShellCommands::ShellReadFile if check_args_len(&args) => shell_read_file(args[1]),
        shell_commands::MommyShellCommands::ShellCreateDir if check_args_len(&args) => shell_create_dir(args[1]),
        shell_commands::MommyShellCommands::ShellDeleteDir if check_args_len(&args) => shell_delete_dir(args[1]),
        shell_commands::MommyShellCommands::ShellChangeCodeDir if check_args_len(&args) => shell_change_code_dir(args[1], mommy_settings),

        // Error
        shell_commands::MommyShellCommands::ShellUnknownCommand => println!("{}", responses::MommyShellError::GeneralInvalid),
        _ => println!("{}", responses::MommyShellError::GeneralInvalid),
    }
}


fn shell_change_code_dir(new_dir: &str, mommy_settings: &mut config::MommySettings) {
    // 1. Get where we are standing right now
    let current_working_dir = env::current_dir().unwrap();

    // 2. Create the FULL PATH (e.g., C:\Users\You\Project\Deep\Folder\Test)
    // We use .join() to combine the current location with the new folder name
    let absolute_target = current_working_dir.join(new_dir);

    // 3. Convert it to a string and save IT
    mommy_settings.output_directory = absolute_target.to_string_lossy().to_string();

    // 4. Save to file
    if let Err(_) = mommy_settings.save() {
        println!("{}", responses::MommyShellError::ConfigSaveError);
    } else {
        println!("{}", responses::MommyShellOk::ConfigUpdated);
        println!("(Mommy set the output to: {})", mommy_settings.output_directory);
    }
}

fn shell_clear(){
    print!("{}", responses::MommyUI::Clear);
     io::stdout().flush().unwrap();
}


fn shell_create_dir(dir_name: &str){
    match fs::create_dir(dir_name) {
        Ok(_) => println!("{}", responses::MommyShellOk::DirectoryCreated),
        Err(_) => println!("{}", responses::MommyShellError::DirectoryNotFound),
    }
}

fn shell_delete_dir(dir_name: &str){
    match fs::remove_dir(dir_name) {
        Ok(_) => println!("{}", responses::MommyShellOk::DirectoryDeleted),
        Err(_) => println!("{}", responses::MommyShellError::DirectoryNotFound),
    }
}


fn shell_run_file(file_name: &str, output_dir: &str) {
    let extension = Path::new(file_name).extension().and_then(|ext| ext.to_str()).
        unwrap_or("");

    match extension {
        "mommy" => {
            // 1. Build the path: "sandbox/test.mommy"
           let base_path = Path::new(output_dir);

            // 2. Use .join() instead of format! (This handles slashes and prefixes)
            let target_path = base_path.join(file_name);

            // 3. Check for existence using the Path object
            if target_path.exists() {
                run_mommy_lang(target_path.to_str().unwrap());
            } else if Path::new(file_name).exists() {
                run_mommy_lang(file_name);
            } else {
                println!("Mommy Error: I cannot find '{}' in '{}' or the current folder.", file_name, output_dir);
            }
        },
        "txt" => {
            simple_exec(constants::CMD_RUN_NOTEPAD, file_name);
        },
        "py" => {
            simple_exec(constants::CMD_RUN_PYTHON, file_name);
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

fn shell_prepare_coding(mommy_settings: &mut config::MommySettings) {
    let mut input = String::new();
    println!("{}", constants::SEPARATOR);
    println!("{}", responses::MommyUI::PrepareCoding);
    println!("{}", responses::MommyUI::WelcomePrompt);
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect(&responses::MommyUI::ExitMessage.to_string());


    match input.trim() {
        "Y" => shell_start_coding(mommy_settings),
        _ => {
            println!("{}", responses::MommyUI::RefuseCoding);
            return;
        },
    }
}

fn shell_start_coding(mommy_settings: &mut config::MommySettings) {
    println!("{}", constants::SEPARATOR);
    println!("{}", responses::MommyUI::StartCoding);
    println!("{}", constants::SEPARATOR);

    let mut lite_ide = String::new();
    let mut line_count = 1;

    loop { // Writing process
        print!("{}. ", line_count);

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect(&responses::MommyShellError::CannotCreateFile.to_string());

        if input.trim() == constants::SHELL_CMD_SAVE {
            break;
        } else if input.trim() == constants:: SHELL_CMD_EXIT {
            return;
        } else if input.trim() == constants::SHELL_CMD_CLEAR {
            lite_ide.clear();
            line_count = 1;
            println!("{}", responses::MommyUI::RestartCLI);
            continue
        }
        lite_ide.push_str(&input);

        line_count += 1;
    }

    shell_save_coding(&lite_ide, mommy_settings);
}

fn shell_save_coding(lite_ide: &str, mommy_settings: &config::MommySettings) {
    println!("{}", constants::SEPARATOR);
    println!("{}", responses::MommyLangStatus::RenameFile);

    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect("Failed to read input");
    let clean_name = input_name.trim();
    let final_filename = validate_file(&clean_name);

    // 1. Get the Absolute Path from settings
    let target_dir_path = Path::new(&mommy_settings.output_directory);

    // 2. Create it if it doesn't exist (Self-Healing)
    if !target_dir_path.exists() {
        if let Err(_) = fs::create_dir_all(target_dir_path) {
            println!("Mommy Error: I tried to go to '{}', but I couldn't create it.", mommy_settings.output_directory);
            return;
        }
    }

    // 3. Join them
    let full_path = target_dir_path.join(final_filename);

    match fs::write(&full_path, lite_ide) {
        Ok(_) => {
            println!("{}", responses::MommyShellOk::FileCreated);
            shell_instant_run_mommy_file(full_path.to_str().unwrap());
        },
        Err(_) => println!("{}", responses::MommyShellError::CannotCreateFile),
    }
}

fn shell_instant_run_mommy_file(full_path: &str) {
    println!("{}", constants::SEPARATOR);
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
    if clean_name.ends_with(constants::EXT_SOURCE) {
        clean_name.to_string()
    } else {
        format!("{}{}", clean_name, constants::EXT_SOURCE)
    }
}

fn run_mommy_lang(filename: &str) {
    println!("{}", constants::SEPARATOR);
    println!("{}", responses::MommyLangStatus::CheckingFile);

    let absolute_path = fs::canonicalize(filename)
        .unwrap_or_else(|_| std::path::PathBuf::from(filename));

    let clean_path = absolute_path.to_string_lossy().replace(
        constants::SHELL_PATH_PREFIX, constants::SHELL_EMPTY);

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

    println!("{}", constants::SEPARATOR);

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







