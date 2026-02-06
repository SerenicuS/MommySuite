use std::env::set_current_dir;
use std::{env, fs};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use mommy_lib::mommy_response;

const SEPARATOR: &str = "----------------------------------------------------------------";

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
     9. letusplayhouse <directory>  ->    Create a Directory
    10. removethehouse <directory>  ->    Delete a Directory
    11. openthis <filename>         ->    Open the File
    12. readthis <filename>         ->    Read the File's contents
    13. doxxme                      ->    Windows Ip Configuration
    14. callmeplease <ip/dns>       ->    Ping device
    15. runthis <filename>          ->    Run File
    ---------------
    "#;


const SHELL_ADVANCE_COMMANDS: &str = r#"
    You are too greedy.
    ---------------
     1. startcoding                 ->    Enter lite_IDE
    ---------------
    "#;




fn main() {
    let root_dir = env::current_dir().expect(&mommy_response::MommyShellError::RootDirError.to_string());

    println!("{}", mommy_response::MommyUI::WelcomeTitle);
    println!("{}", mommy_response::MommyUI::WelcomeSubtitle);
    println!("{}", mommy_response::MommyUI::WelcomePrompt);

    loop{

        let mut input = String::new();// lineBuffer
        println!();
        io::stdin().read_line(&mut input).expect(&mommy_response::MommyUI::ExitMessage.to_string());

        match input.trim(){
            "Y" => shell_start_default(input, &root_dir),
            _ => std::process::exit(0),
        }
    }

}

fn shell_start_default(mut input: String, root_dir: &std::path::PathBuf) { // Added root_dir
    println!("{}", mommy_response::MommyUI::GenericObedience);

    loop {
        input.clear();
        print!(">");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect(&mommy_response::MommyShellError::GeneralInvalid.to_string());

        println!("{}", SEPARATOR);

        // Pass the root_dir down the chain
        shell_attempt_command(&input, root_dir);
    }
}

fn shell_open_file(file_name: &str){
    match Command::new("cmd").args(&["/C", "start", file_name]).output(){
        Ok(_) => println!("{}", mommy_response::MommyShellOk::FileOpened),
        Err(_) => println!("{}", mommy_response::MommyShellError::FileNotFound),
    }
}

fn shell_return_to_prev_directory(root_dir: &std::path::PathBuf) {
    let current_dir = env::current_dir().unwrap();

    if current_dir.canonicalize().unwrap() == root_dir.canonicalize().unwrap() {
        println!("{}", mommy_response::MommyShellError::RootDirectoryLocked);
    } else {
        match set_current_dir("..") {
            Ok(_) => println!("{}", mommy_response::MommyShellOk::DirectoryChanged),
            Err(_) => println!("{}", mommy_response::MommyShellError::GeneralInvalid),
        }
    }
}


fn check_args_len(args: &Vec<&str>) -> bool {
        args.len() > 1
    }

    fn shell_create_file(file_name: &str) {
        match fs::File::create(file_name) {
            Ok(_) => println!("{}", mommy_response::MommyShellOk::FileCreated),
            Err(_) => println!("{}", mommy_response::MommyShellError::CannotCreateFile)
        }
    }

    fn shell_delete_file(file_name: &str) {
        match fs::remove_file(file_name) {
            Ok(_) => println!("{}", mommy_response::MommyShellOk::FileDeleted),
            Err(_) => println!("{}", mommy_response::MommyShellError::CannotDeleteFile),
        }
    }
    fn shell_list_files_in_directory() {
        let files = fs::read_dir(".").expect(&mommy_response::MommyShellError::CannotListFiles.to_string());

        for entry in files {
            let entry = entry.expect(&mommy_response::MommyShellError::CannotListFiles.to_string());
            println!("{}", entry.path().display());
        }
    }
    fn shell_get_directory() {
        let dir = env::current_dir().expect(&mommy_response::MommyShellError::DirectoryNotFound.to_string());
        println!("{}", dir.display());
    }

    fn shell_get_directory_return() -> String {
        let dir = env::current_dir().expect(&mommy_response::MommyShellError::DirectoryNotFound.to_string());

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
                        println!("Moved Inside: {}", shell_get_directory_return());
                    }
                } else {
                    println!("{}", mommy_response::MommyShellError::RootDirectoryLocked);
                }
            }
            Err(_) => println!("{}", mommy_response::MommyShellError::DirectoryNotFound),
        }
    }



fn shell_attempt_command(input: &str, root_dir: &std::path::PathBuf) {
    let clean_input = input.trim();
    let args: Vec<&str> = clean_input.split_whitespace().collect();

    if args.is_empty() {
        println!("{}", mommy_response::MommyShellError::IncompleteArgs);
        return;
    }

    match args[0] {
        //1 Args
        "tellme" => shell_print_basic_help(),
        "tellmesecret" => shell_print_advance_help(),
        "mayileave" => std::process::exit(0),
        "iamhere" => shell_get_directory(),
        "mommy?" => shell_list_files_in_directory(),
        "doxxme" => shell_windows_call("ipconfig"),
        "goback" => shell_return_to_prev_directory(root_dir),


        //Advanced
        "startcoding" => shell_prepare_coding(),

        // 2 Args
        "walkwithme" if check_args_len(&args) => shell_move_directory(args[1], root_dir),
        "canihave" if check_args_len(&args) => shell_create_file(args[1]),
        "takethe" if check_args_len(&args) => shell_delete_file(args[1]),
        "openthis" if check_args_len(&args) => shell_open_file(args[1]),
        "runthis" if check_args_len(&args) => shell_run_file(args[1]),
        _ => println!("{}", mommy_response::MommyShellError::GeneralInvalid),
    }



    fn shell_run_file(filename: &str) {
        let extension = Path::new(filename).extension().and_then(|ext| ext.to_str()).
            unwrap_or("");

        match extension { //file type selection NOTE: it does not run without specifying the name
            "mommy" => {
                run_mommy_lang(filename); // Running files that end with .mommy
            },
            "txt" => {
                simple_exec("notepad.exe", filename);
            },
            "py" => {
                simple_exec("python", filename);
            },
            _ => {
                println!("{}", mommy_response::MommyShellError::CannotOpenFile)
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
        println!("{}", mommy_response::MommyUI::PrepareCoding);
        println!("{}", mommy_response::MommyUI::WelcomePrompt);
        input.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect(&mommy_response::MommyUI::ExitMessage.to_string());


        match input.trim() {
            "Y" => shell_start_coding(),
            _ => {
                println!("{}", mommy_response::MommyUI::RefuseCoding);
                return;
            },
        }
    }

    fn shell_start_coding() {
        println!("{}", SEPARATOR);
        println!("{}", mommy_response::MommyUI::StartCoding);
        println!("{}", SEPARATOR);

        let mut lite_ide = String::new();
        let mut line_count = 1;

        loop { // Writing process
            print!("{}. ", line_count);

            io::stdout().flush().unwrap();

            let mut input = String::new();

            io::stdin().read_line(&mut input).expect(&mommy_response::MommyShellError::CannotCreateFile.to_string());

            if input.trim() == "SAVE" {
                break;
            } else if input.trim() == "EXIT" {
                return;
            } else if input.trim() == "CLEAR" {
                lite_ide.clear();
                line_count = 1;
                println!("{}", mommy_response::MommyUI::RestartCLI);
                continue
            }
            lite_ide.push_str(&input);

            line_count += 1;
        }

        shell_save_coding(&lite_ide);
    }

    fn shell_save_coding(lite_ide: &str) {
        println!("{}", SEPARATOR);
        println!("{}", mommy_response::MommyLangStatus::RenameFile);

        let mut input_name = String::new();
        io::stdin()
            .read_line(&mut input_name)
            .expect("Failed to read input");

        let clean_name = input_name.trim();

        let final_filename = validate_file(&clean_name);

        // Without this, fs::write crashes on a fresh install.
        let sandbox_dir = "sandbox";
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
                println!("{}", mommy_response::MommyShellOk::FileCreated);
                // Run the file immediately so they see their results
                shell_instant_run_mommy_file(&full_path);
            },
            Err(_) => println!("{}", mommy_response::MommyShellError::CannotCreateFile),
        }
    }

    fn shell_instant_run_mommy_file(full_path: &str) {
        println!("{}", SEPARATOR);
        println!("{}", mommy_response::MommyLangStatus::PrepareRun);
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();


        if ans.trim().eq_ignore_ascii_case("Y") { // Run it immediately
            run_mommy_lang(&full_path);
        } else {
            println!("{}", mommy_response::MommyLangStatus::SaveOnly);
            return
        }
    }

    fn validate_file(clean_name: &str) -> String {
        if clean_name.ends_with(".mommy") {
            clean_name.to_string()
        } else {
            format!("{}.mommy", clean_name)
        }
    }

    fn run_mommy_lang(filename: &str) {
        println!("{}", SEPARATOR);


        let (cmd, args) = if cfg!(debug_assertions) {
            // DEV MODE: Use Cargo
            println!("[DEBUG] Running via Cargo...");
            ("cargo".to_string(), vec!["run", "-p", "mommy_lang", "--", filename])
        } else {
            // PROD MODE: Locate sibling executable
            // Get the path where 'mommy_shell.exe' actually lives
            let mut path = env::current_exe().expect("Unable to get current process path");
            path.pop();

            // Append 'mommy_lang.exe'
            if cfg!(target_os = "windows") {
                path.push("mommy_lang.exe");
            } else {
                path.push("mommy_lang");
            }

            if !path.exists() {
                println!("Mommy Error: I cannot find 'mommy_lang.exe'. We need to be in the same folder, sweetie.");
                return;
            }

            println!("[RELEASE] Running Mommy Compiler...");
            (path.to_string_lossy().to_string(), vec![filename])
        };

        let status_result = Command::new(cmd)
            .args(&args)
            .status();

        println!("{}", SEPARATOR);

        match status_result {
            Ok(status) if status.success() => println!("{}", mommy_response::MommyLangStatus::ResultOk),
            Err(_) => println!("{}", mommy_response::MommyLangStatus::ResultError),
            _ => println!("{}", mommy_response::MommyLangStatus::ResultError),
        }
    }

    /*

        WINDOWS COMMANDS
     */

    fn shell_windows_call(windows_command: &str) {
        match windows_command {
            "ipconfig" => windows_command_console_output(windows_command),
            _ => println!("{}", mommy_response::MommyShellError::ExternalIPConfigCallFail),
        }
    }

    fn windows_command_console_output(var: &str) {
        match Command::new(var).output() {
            Ok(output) => {
                let console_output = String::from_utf8_lossy(&output.stdout);
                println!("{}", console_output);
            }
            Err(_) => println!("{}", mommy_response::MommyShellError::ExternalCommandFailed),
        }
    }
}



