use std::env::set_current_dir;
use std::{fmt, fs};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub enum ShellOkResponse {
    OkGeneral,
    OkDeleteFile,
    OkCreateFile,
    OkReturnDirectory,
    OkListedFiles,
    OkMoveDirectory,
    OkTerminate,
    OkCreateDirectory,
    OkDeleteDirectory,
    OkReadFile,
    OkOpenedFile,
    OkLaunchProcess,
}

pub enum ShellErrorResponse {
    ErrorGeneral,
    ErrorBadArgs,
    ErrorTooManyArgs,
    ErrorSystem,
    ErrorFileDoesNotExist,
    ErrorPermissionDenied,
    ErrorRootDirectory,
    ErrorListedFilesDoesNotExist,
    ErrorProcessDoesNotExist,
    ErrorIncompleteLaunchProcess,
    ErrorDirectoryDoesNotExist,
}

pub enum OkFlavorResponse {
    FlavorIpConfigAttempt,
    FlavorPingAttempt,
}

pub enum BadFlavorResponse {
    FlavorWindowsCallFail,
    FlavorWindowsCommandFail,
    FlavorWindowsConsoleFail,
}

pub enum GeneralFlavorResponse {
    FlavorMenu1,
    FlavorMenu2,
    FlavorMenu3,
    FlavorExit,
    FlavorRegister1,
    FlavorRegister2,
    FlavorStart1,
    FlavorChaosNotHear,
    FlavorChaosWrongCommand,
    FlavorStartCoding,
    FlavorRefuseCoding

}

pub enum MiscFlavor {
    FlavorWhitespace1,
    FlavorWhiteSpace2,
    FlavorUserReply1,
}

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


impl fmt::Display for OkFlavorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FlavorIpConfigAttempt => write!(f, "{}", "Do not tell others about our location sweetie, you only need to rely on me."),
            Self::FlavorPingAttempt => write!(f, "{}", "Are you calling someone sweetie? You do know that we only rely on each other."),


        }
    }
}

impl fmt::Display for BadFlavorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FlavorWindowsCallFail => write!(f, "{}", "Your friend did not respond to your calls?"),
            Self::FlavorWindowsCommandFail => write!(f, "{}", "Your friend did not like how you commanded him. You want to make him obey?"),
            Self::FlavorWindowsConsoleFail => write!(f, "{}", "Your friend cannot write because i broke his fingers, sorry sweetie."),
        }
    }
}

impl fmt::Display for ShellErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::ErrorGeneral => write!(f, "Tell me the instructions correctly sweetie."),
            Self::ErrorBadArgs => write!(f, "You didn't complete your sentence sweetie, are you flustered?."),
            Self::ErrorTooManyArgs => write!(f, "Greedy Aren't you?."),
            Self::ErrorSystem => write!(f, "Oh my, the system crashed."),
            Self::ErrorFileDoesNotExist => write!(f, "You are not allowed to do that sweetie?"),
            Self::ErrorPermissionDenied => write!(f, "This is as far as we can go sweetie."),
            Self::ErrorRootDirectory => write!(f, "Hmmm, no one is here, only your mommy right?."),
            Self::ErrorListedFilesDoesNotExist => write!(f, "Hmmm, no one is here, only your mommy right?."),
            Self::ErrorProcessDoesNotExist => write!(f, "What kind of action you want me to do sweetie? Say it properly."),
            Self::ErrorIncompleteLaunchProcess => write!(f, "I can't do it properly if you won't say clearly what you desire sweetie."),
            Self::ErrorDirectoryDoesNotExist => write!(f, "I cannot find the house sweetie."),

        }
    }
}


impl fmt::Display for GeneralFlavorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FlavorMenu1 => write!(f, "Hello To my Custom Shell!"),
            Self::FlavorMenu2 => write!(f, "It is made by \"HiveMind\" to showcase my talents ^^."),
            Self::FlavorMenu3 => write!(f, "Press Y(Manipulation) or Z(Default) key to start using it. "),
            Self::FlavorExit => write!(f, "Exiting....."),
            Self::FlavorRegister1 => write!(f, "Do you know your name?"),
            Self::FlavorRegister2 => write!(f, "Tell me your name sweetie.. "),
            Self::FlavorStart1 => write!(f, "Good boy, always listen to your mommy."),
            Self::FlavorChaosNotHear => write!(f, "Are you talking sweetie? I did not hear you. Can you repeat that again?"),
            Self::FlavorChaosWrongCommand => write!(f, "You already told me that, you are so impatient sweetie."),
            Self::FlavorStartCoding => write!(f, "Do you want to instruct me sweetie?"),
            Self::FlavorRefuseCoding => write!(f, "Why did you told me to prepare it sweetie? You are wasting my time.")
        }
    }
}

impl fmt::Display for ShellOkResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{

            Self::OkGeneral => write!(f, "It was successful sweetie."),
            Self::OkDeleteFile => write!(f, "You don't like this? Fine, I will have it."),
            Self::OkCreateFile => write!(f, "Here sweetie, please take care of it."),
            Self::OkReturnDirectory => write!(f, "Be careful sweetie."),
            Self::OkListedFiles => write!(f, "You don't trust your mommy?..."),
            Self::OkMoveDirectory => write!(f, "We are here now, do you like it?"),
            Self::OkTerminate => write!(f, "Talk to you later sweetie."),
            Self::OkCreateDirectory => write!(f, "Oh, you want to play house with me sweetie?"),
            Self::OkDeleteDirectory => write!(f, "You don't have to do that, we can just create more house."),
            Self::OkReadFile => write!(f, "Do you like the contents of the file sweetie?"),
            Self::OkOpenedFile => write!(f, "Write what is important for you, sweet boy."),
            Self::OkLaunchProcess => write!(f, "Are you satisfied sweetie?."),

        }
    }
}


fn main() {
    println!("{}", GeneralFlavorResponse::FlavorMenu1);
    println!("{}", GeneralFlavorResponse::FlavorMenu2);
    println!("{}", GeneralFlavorResponse::FlavorMenu3);

    loop{

        let mut input = String::new();// lineBuffer
        println!();
        io::stdin().read_line(&mut input).expect(&GeneralFlavorResponse::FlavorExit.to_string());

        match input.trim(){
            "Y" => shell_start_default(input),
            "N" => std::process::exit(0),
            _ => println!("{}", ShellErrorResponse::ErrorGeneral),
        }
    }

}

fn shell_start_default(mut input: String){
    println!("{}", GeneralFlavorResponse::FlavorStart1);

    loop{
        input.clear();
        print!(">");
        io::stdout().flush().unwrap(); // This exists because without this, the ">" will not show up and get stuck.
        io::stdin().read_line(&mut input).expect(&ShellErrorResponse::ErrorGeneral.to_string());

        shell_attempt_command(&input)

    }
}

fn shell_open_file(file_name: &str){
    match Command::new("cmd").args(&["/C", "start", file_name]).output(){
        Ok(_) => println!("{}", ShellOkResponse::OkOpenedFile),
        Err(_) => println!("{}", ShellErrorResponse::ErrorFileDoesNotExist),
    }
}

fn shell_return_to_prev_directory(){


    match set_current_dir(".."){
        Ok(_) => println!("{}", ShellOkResponse::OkReturnDirectory),
        Err(_) => println!("{}", ShellErrorResponse::ErrorRootDirectory),
    }
}


fn check_args_len(args: &Vec<&str>) -> bool{
    args.len() > 1
}


fn shell_create_file(file_name: &str){
    match fs::File::create(file_name){
        Ok(_) => println!("{}", ShellOkResponse::OkCreateFile),
        Err(_) => println!("{}", ShellErrorResponse::ErrorPermissionDenied)
    }
}

fn shell_delete_file(file_name: &str){
    match fs::remove_file(file_name){
        Ok(_) => println!("{}", ShellOkResponse::OkDeleteFile),
        Err(_) => println!("{}", ShellErrorResponse::ErrorFileDoesNotExist)
    }
}
fn shell_list_files_in_directory(){
    let files = fs::read_dir(".").expect(&ShellErrorResponse::ErrorListedFilesDoesNotExist.to_string());

    for entry in files{
        let entry = entry.expect(&ShellErrorResponse::ErrorPermissionDenied.to_string());
        println!("{}", entry.path().display());
    }
}
fn shell_get_directory(){
    let dir = std::env::current_dir().expect(&ShellErrorResponse::ErrorRootDirectory.to_string());
    println!("{}", dir.display());

}


fn shell_get_directory_return() -> String{
    let dir = std::env::current_dir().expect(&ShellErrorResponse::ErrorRootDirectory.to_string());

    dir.display().to_string()

}
fn shell_help(){
    println!("{}", HELP_MENU);
}

fn shell_move_directory(path: &str){
    match set_current_dir(path){
        Ok(_) => println!("Moved Inside: {}", shell_get_directory_return()),
        Err(_) => println!("{}", ShellErrorResponse::ErrorSystem),
    }
}

fn shell_attempt_command(input: &str){
    let clean_input = input.trim();
    let args: Vec<&str> = clean_input.split_whitespace().collect();

    if args.is_empty(){
        println!("{}", ShellErrorResponse::ErrorBadArgs);
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
        _ => println!("{}", ShellErrorResponse::ErrorGeneral),
    }

}

fn shell_run_file(filename: &str) {
    // 1. Get the extension
    let extension = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    println!("Mommy is checking how to open .{} files...", extension);

    // 2. Execute based on extension
    match extension {
        "mommy" => {
            // OPTION A: The Special One (Your Compiler)
            // This function already contains the Command::new logic!
            run_mommy_lang(filename);
        },
        "txt" => {
            // OPTION B: Simple Tools (Notepad)
            simple_exec("notepad.exe", filename);
        },
        "py" => {
            // OPTION C: Python
            simple_exec("python", filename);
        },
        _ => {
            println!("Mommy doesn't know how to open .{} files! 💅", extension);
        }
    }
}

// A tiny helper function for the "boring" tools like Notepad/Python
fn simple_exec(tool: &str, filename: &str) {
    println!("Opening {} with {}...", filename, tool);
    Command::new(tool)
        .arg(filename)
        .status()
        .expect("Failed to run the command");
}


fn run_mommy_lang(filename: &str) {
    println!("Mommy Shell: Handing over '{}' to the compiler...", filename);

    let status_result = if cfg!(debug_assertions) {
        // 🛠️ DEV MODE (Automatic Rebuilds)
        // This is magic: It tells Cargo to run the OTHER project in your workspace.
        // It ensures mommy_lang is recompiled before running!
        println!("[DEBUG] Running via Cargo...");
        Command::new("cargo")
            .args(["run", "-p", "mommy_lang", "--", filename])
            .status()
    } else {
        // 🚀 RELEASE MODE (Manual .exe)
        // This is for when you eventually zip it up for users.
        let exe_name = if cfg!(target_os = "windows") { "mommy_lang.exe" } else { "mommy_lang" };
        println!("[RELEASE] Running {}...", exe_name);
        Command::new(format!("./{}", exe_name))
            .arg(filename)
            .status()
    };

    match status_result {
        Ok(status) if status.success() => println!("Mommy Shell: Compilation success! ✨"),
        Ok(_) => println!("Mommy Shell: The compiler ran but found errors in your code. 💅"),
        Err(_) => println!("Mommy Shell: CRITICAL ERROR - Could not find the compiler!"),
    }
}


fn shell_prepare_coding(){
    let mut input = String::new();
    println!("{}", GeneralFlavorResponse::FlavorStartCoding);
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect(&GeneralFlavorResponse::FlavorExit.to_string());

    // Preparing the directory
    match input.trim(){
        "Y" => shell_prepare_coding_directory(),
        "N" => {
            println!("{}", GeneralFlavorResponse::FlavorRefuseCoding);
            return;
        },
        _ =>{
            println!("{}", ShellErrorResponse::ErrorBadArgs);
            return;
        },
    }

}

fn shell_prepare_coding_directory(){
    let mut input = String::new();

    println!("{}", "Wait sweetie, let me prepare the environment first.");
    print!("{}", "Should we prepare the table first?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect(&GeneralFlavorResponse::FlavorExit.to_string());

    match input.trim(){
        "Y" => shell_start_coding(),
        "N" => println!("So you want to be lazy? Fine sweetie."),
        _ => println!("{}", ShellErrorResponse::ErrorBadArgs),
    }

}


fn shell_start_coding() {
    println!("Alright sweetie, start typing. Type 'SAVE' when you are done.");

    // This string will hold the ENTIRE file content
    let mut lite_ide = String::new();
    let mut line_count = 1; // Start at 1, it looks nicer than 0

    loop {
        // 1. Print the Line Number (e.g., "1. ")
        print!("{}. ", line_count);
        io::stdout().flush().unwrap(); // Push number to screen immediately

        // 2. Get Input
        let mut input = String::new(); // New buffer for THIS line only
        io::stdin().read_line(&mut input).expect("Failed to read");

        // 3. Check for Exit Command
        // We trim() to remove the enter key (\n) at the end
        if input.trim() == "SAVE" {
            println!("Good job! Saving your work...");
            break; // Break the loop
        }

        // 4. Append the line to our "File" variable
        lite_ide.push_str(&input);

        // 5. Increment line counter
        line_count += 1;
    }

    println!("--- SAVING TO DISK ---");

    println!("Name of your work sweetie? (e.g. 'math')");
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect("Failed to read");

    // FIX 1: Clean the input (Remove the Enter key)
    let clean_name = input_name.trim();

    // FIX 2: Check if user already typed ".mommy"
    // We want "test" -> "test.mommy", but "test.mommy" -> "test.mommy" (not .mommy.mommy)
    let final_filename = if clean_name.ends_with(".mommy") {
        clean_name.to_string()
    } else {
        format!("{}.mommy", clean_name)
    };

    // FIX 3: Use format! for the path and fix the spelling of 'sandbox'
    let full_path = format!("sandbox/{}", final_filename);

    println!("Saving to {}...", full_path);

    // FIX 4: Write using the formatted path
    match fs::write(&full_path, lite_ide) {
        Ok(_) => {
            println!("Saved successfully! ✨");
            // BONUS: Run it immediately?
            println!("Would you like to run it now? (Y/N)");
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).unwrap();
            if ans.trim().eq_ignore_ascii_case("Y") {
                // Pass just the relative path: "sandbox/test.mommy"
                run_mommy_lang(&full_path);
            }
        },
        Err(e) => println!("Mommy failed to save file: {}", e),
    }
}



/*

    WINDOWS COMMANDS
 */



fn shell_windows_call(windows_command: &str){
    match windows_command{
        "ipconfig" => windows_command_console_output(windows_command),
        _ => println!("{}", BadFlavorResponse::FlavorWindowsCallFail),
    }
}

fn windows_command_console_output(var: &str) {
    match Command::new(var).output() {
        Ok(output) => {
            let console_output = String::from_utf8_lossy(&output.stdout);
            println!("{}", console_output);
        }
        Err(_) => println!("{}", BadFlavorResponse::FlavorWindowsCommandFail),
    }
}



/*

TODO LIST (For Tomorrow)

1. Refactor Mommy Lang [AI(code temporary) -> Clean Arch]

    [ ] Create a Config struct to handle path generation (input, .c, .exe) in one place.

    [ ] Split main.rs into clear pipeline steps: transpile(), compile_gcc(), run_exe().

    [ ] Add better exit codes (don't try to run GCC if transpilation failed).

2. Refactor Mommy Shell [AI(code temporary) -> Clean Arch]

    [ ] Split the massive shell_start_coding function.

    [ ] Create run_lite_editor() (Handles just the typing loop).

    [ ] Create run_save_dialog() (Handles file naming and saving).

    [ ] Ensure sandbox directory is created if it doesn't exist.

3. Library Cleanup

    [ ] Critical: Remove duplicate error handling code in Shell.

    [ ] Move all Error definitions to mommy_lib (Shared Brain).

4. Quality Assurance

    [ ] Test the "Release Build" (running the EXEs outside of the IDE).
 */