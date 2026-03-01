use std::env::set_current_dir;
use std::fs;


const EXE_EXTRACT_TO_BIN: &[(&str, &[u8])] = &[
    ("mommy_shell.exe", include_bytes!("../../target/release/mommy_shell.exe")),
    ("mommy_lang.exe", include_bytes!("../../target/release/mommy_lang.exe")),
    ("mommy_editor.exe", include_bytes!("../.././mommy_editor/mommy_editor.exe")),
];

const SUITE_EXTRACT_TO_ROOT: &[(&str, &[u8])] = &[
    ("mommy_suite.exe", include_bytes!("../../target/release/mommy_suite.exe")),
];



const DIR_TO_CREATE: &[&str] = &[
    "mommy_brain",
    "mommy_trash",
    "mommy_properties",
    "sandbox",
    "mommy_bin",

];

fn main() {

    println!("Mommy Suite Installer v0.9.3");
    println!("================================");

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Dropping files to: {}", current_dir.display());
    println!();


    println!("Dropping dir files...");
    for filename in DIR_TO_CREATE {
        print!("  - {}... ", filename);
        match fs::create_dir_all(filename){
            Ok(_) => println!(),
            Err(e) => println!("Failed: {}", e),
        }
    }



    set_current_dir(DIR_TO_CREATE[4]).unwrap();
    println!("Dropping exe files to: {}", DIR_TO_CREATE[4]);
    for (filename, data) in EXE_EXTRACT_TO_BIN {
        print!("  - {}... ", filename);
        match fs::write(filename, data) {
            Ok(_) => println!(),
            Err(e) => println!("Failed: {}", e),
        }
    }

    set_current_dir("../").unwrap();
    println!("Dropping main file to: {}", current_dir.display());
    match fs::write(SUITE_EXTRACT_TO_ROOT[0].0, SUITE_EXTRACT_TO_ROOT[0].1) {
        Ok(_) => println!("  - {}... ", SUITE_EXTRACT_TO_ROOT[0].0),
        Err(e) => println!("Failed: {}", e),
    }


    println!("\n Necessary files are dropped successfully!");
    println!("================================");
    println!("Run 'mommy_suite.exe' to start the OS");
    println!("The OS will create its own directories");
    println!();

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}