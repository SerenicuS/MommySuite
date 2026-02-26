use std::fs;

const FILES_TO_EXTRACT: &[(&str, &[u8])] = &[
    ("mommy_shell.exe", include_bytes!("../../target/release/mommy_shell.exe")),
    ("mommy_lang.exe", include_bytes!("../../target/release/mommy_lang.exe")),
    ("mommy_editor.exe", include_bytes!("../.././mommy_editor/mommy_editor.exe")),
    ("mommy_suite.exe", include_bytes!("../../target/release/mommy_suite.exe")),
];

fn main() {
    println!("Mommy Suite Installer v1.0");
    println!("================================");

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Dropping files to: {}", current_dir.display());
    println!();

    println!("Dropping files...");
    for (filename, data) in FILES_TO_EXTRACT {
        print!("  - {}... ", filename);
        match fs::write(current_dir.join(filename), data) {
            Ok(_) => println!(""),
            Err(e) => println!("Failed: {}", e),
        }
    }


    println!("\n Files dropped successfully!");
    println!("================================");
    println!("Run 'mommy_suite.exe' to start the OS");
    println!("The OS will create its own directories");
    println!();

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}