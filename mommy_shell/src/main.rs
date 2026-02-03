use std::env::set_current_dir;
use std::{fs};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use mommy_lib::mommy_response;


const HELP_MENU: &str = r#"
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




fn main() {
    println!("{}", mommy_response::GeneralFlavorResponse::FlavorMenu1);
    println!("{}", mommy_response::GeneralFlavorResponse::FlavorMenu2);
    println!("{}", mommy_response::GeneralFlavorResponse::FlavorMenu3);

    loop{

        let mut input = String::new();// lineBuffer
        println!();
        io::stdin().read_line(&mut input).expect(&mommy_response::GeneralFlavorResponse::FlavorExit.to_string());

        match input.trim(){
            "Y" => shell_start_default(input),
            _ => std::process::exit(0),
        }
    }

}

fn shell_start_default(mut input: String){
    println!("{}", mommy_response::GeneralFlavorResponse::FlavorStart1);

    loop{
        input.clear();
        print!(">");
        io::stdout().flush().unwrap(); // This exists because without this, the ">" will not show up and get stuck.
        io::stdin().read_line(&mut input).expect(&mommy_response::ShellErrorResponse::ErrorGeneral.to_string());

        shell_attempt_command(&input)

    }
}

fn shell_open_file(file_name: &str){
    match Command::new("cmd").args(&["/C", "start", file_name]).output(){
        Ok(_) => println!("{}", mommy_response::ShellOkResponse::OkOpenedFile),
        Err(_) => println!("{}", mommy_response::ShellErrorResponse::ErrorFileDoesNotExist),
    }
}

fn shell_return_to_prev_directory(){
    match set_current_dir(".."){
        Ok(_) => println!("{}", mommy_response:: ShellOkResponse::OkReturnDirectory),
        Err(_) => println!("{}", mommy_response::ShellErrorResponse::ErrorRootDirectory),
    }
}


fn check_args_len(args: &Vec<&str>) -> bool{
    args.len() > 1
}


fn shell_create_file(file_name: &str){
    match fs::File::create(file_name){
        Ok(_) => println!("{}", mommy_response::ShellOkResponse::OkCreateFile),
        Err(_) => println!("{}", mommy_response::ShellErrorResponse::ErrorPermissionDenied)
    }
}

fn shell_delete_file(file_name: &str){
    match fs::remove_file(file_name){
        Ok(_) => println!("{}", mommy_response::ShellOkResponse::OkDeleteFile),
        Err(_) => println!("{}", mommy_response::ShellErrorResponse::ErrorFileDoesNotExist)
    }
}
fn shell_list_files_in_directory(){
    let files = fs::read_dir(".").expect(&mommy_response::ShellErrorResponse::ErrorListedFilesDoesNotExist.to_string());

    for entry in files{
        let entry = entry.expect(&mommy_response::ShellErrorResponse::ErrorPermissionDenied.to_string());
        println!("{}", entry.path().display());
    }
}
fn shell_get_directory(){
    let dir = std::env::current_dir().expect(&mommy_response::ShellErrorResponse::ErrorRootDirectory.to_string());
    println!("{}", dir.display());

}


fn shell_get_directory_return() -> String{
    let dir = std::env::current_dir().expect(&mommy_response::ShellErrorResponse::ErrorRootDirectory.to_string());

    dir.display().to_string()

}
fn shell_help(){
    println!("{}", HELP_MENU);
}

fn shell_move_directory(path: &str){
    match set_current_dir(path){
        Ok(_) => println!("Moved Inside: {}", shell_get_directory_return()),
        Err(_) => println!("{}", mommy_response::ShellErrorResponse::ErrorSystem),
    }
}

fn shell_attempt_command(input: &str){
    let clean_input = input.trim();
    let args: Vec<&str> = clean_input.split_whitespace().collect();

    if args.is_empty(){
        println!("{}", mommy_response::ShellErrorResponse::ErrorBadArgs);
        return;
    }

    match args[0]{
        //1 Args
        "tellme" => shell_help(),
        "mayileave" => std::process::exit(0),
        "iamhere" => shell_get_directory(),
        "mommy?" => shell_list_files_in_directory(),
        "doxxme" => shell_windows_call("ipconfig"),
        "goback" => shell_return_to_prev_directory(),
        "startcoding" => shell_prepare_coding(),


        // 2 Args
        "walkwithme" if check_args_len(&args) => shell_move_directory(args[1]),
        "canihave" if check_args_len(&args) => shell_create_file(args[1]),
        "takethe" if check_args_len(&args) => shell_delete_file(args[1]),
        "openthis" if check_args_len(&args) => shell_open_file(args[1]),
        "runthis" if check_args_len(&args) => shell_run_file(args[1]),
        _ => println!("{}", mommy_response::ShellErrorResponse::ErrorGeneral),
    }

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
            println!("{}", mommy_response::ShellErrorResponse::ErrorCannotOpenFile)
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

fn shell_prepare_coding(){
    let mut input = String::new();
    println!("{}", mommy_response::GeneralFlavorResponse::FlavorPrepareCoding);
    println!("{}", mommy_response::GeneralFlavorResponse::FlavorMenu3);
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect(&mommy_response::GeneralFlavorResponse::FlavorExit.to_string());


    match input.trim(){
        "Y" => shell_start_coding(),
        _ =>{
            println!("{}", mommy_response::GeneralFlavorResponse::FlavorRefuseCoding);
            return;
        },
    }

}


fn shell_start_coding() {
    println!("{}", mommy_response::GeneralFlavorResponse::FlavorStartCoding);

    let mut lite_ide = String::new();
    let mut line_count = 1;

    loop { // Writing process
        print!("{}. ", line_count);

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");

        if input.trim() == "SAVE" {
            break;
        }
        else if input.trim() == "EXIT"{
            return;
        }
        lite_ide.push_str(&input);

        line_count += 1;
    }

    shell_save_coding(&lite_ide);

}

fn shell_save_coding(lite_ide: &str){
    println!("{}", mommy_response::MommyLangGeneralResponse::RenameFile);
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect(&mommy_response::ShellErrorResponse::ErrorCreateFile.to_string());
    let clean_name =  input_name.trim();

    let final_filename = {
        validate_file(&clean_name)   // if the user added .mommy in the filename, if yes do not try to add another one
    };


    let full_path = format!("sandbox/{}", final_filename);

    match fs::write(&full_path, lite_ide) {
        Ok(_) => {
            println!("{}", mommy_response::ShellOkResponse::OkCreateFile);
            shell_instant_run_mommy_file(&full_path)

        },
        Err(e) => println!("Mommy failed to save file: {}", e),
    }
}


fn shell_instant_run_mommy_file(full_path: &str){
    println!("{}", mommy_response::MommyLangGeneralResponse::PrepareRun);
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();
    if ans.trim().eq_ignore_ascii_case("Y") { // Run it immediately
        run_mommy_lang(&full_path);
    }
    else{
        println!("{}", mommy_response::MommyLangGeneralResponse::SaveOnly);
        return
    }
}



fn validate_file(clean_name: &str) -> String{
    if clean_name.ends_with(".mommy") {
        clean_name.to_string()
    }
    else{
        format!("{}.mommy", clean_name)
    }

}


fn run_mommy_lang(filename: &str) {
    let status_result = if cfg!(debug_assertions) { // Dev mode
        // 🛠️ DEV MODE (Automatic Rebuilds)
        // It ensures mommy_lang is recompiled before running!
        println!("[DEBUG] Running via Cargo...");
        Command::new("cargo")
            .args(["run", "-p", "mommy_lang", "--", filename])
            .status()
    } else {
        // 🚀 RELEASE MODE (Manual .exe)
        let exe_name = if cfg!(target_os = "windows") { "mommy_lang.exe" } else { "mommy_lang" };
        println!("[RELEASE] Running {}...", exe_name);
        Command::new(format!("./{}", exe_name))
            .arg(filename)
            .status()
    };

    match status_result {
        Ok(status) if status.success() => println!("{}", mommy_response::MommyLangGeneralResponse::StatusResultOk),
        Ok(_) => println!("{}", mommy_response::MommyLangGeneralResponse::StatusResultOk),
        Err(_) => println!("{}", mommy_response::MommyLangGeneralResponse::StatusResultError),
    }
}




/*

    WINDOWS COMMANDS
 */



fn shell_windows_call(windows_command: &str){
    match windows_command{
        "ipconfig" => windows_command_console_output(windows_command),
        _ => println!("{}", mommy_response::BadFlavorResponse::FlavorWindowsCallFail),
    }
}

fn windows_command_console_output(var: &str) {
    match Command::new(var).output() {
        Ok(output) => {
            let console_output = String::from_utf8_lossy(&output.stdout);
            println!("{}", console_output);
        }
        Err(_) => println!("{}", mommy_response::BadFlavorResponse::FlavorWindowsCommandFail),
    }
}



/*

TODO LIST (For Tomorrow)

1. Refactor Mommy Lang [AI(code temporary) -> Clean Arch]

    [ ] Create a Config struct to handle path generation (input, .c, .exe) in one place.

    [?] Split main.rs into clear pipeline steps: transpile(), compile_gcc(), run_exe().

    [?] Add better exit codes (don't try to run GCC if transpilation failed).

2. Refactor Mommy Shell [AI(code temporary) -> Clean Arch]

    [/] Split the massive shell_start_coding function.

    [/] Create run_lite_editor() (Handles just the typing loop).

    [/] Create run_save_dialog() (Handles file naming and saving).

    [/] Ensure sandbox directory is created if it doesn't exist.

3. Library Cleanup

    [/] Critical: Remove duplicate error handling code in Shell.

    [/] Move all Error definitions to mommy_lib (Shared Brain).

4. Quality Assurance

    [ ] Test the "Release Build" (running the EXEs outside of the IDE).

5. MommyLang issues

    [ ] Cannot do nested loops yet
    [ ] If conditions cannot house complex syntax
    [ ] Missing Assignment Operator
 */

