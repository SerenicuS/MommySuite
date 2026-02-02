use mommy_lib::syntax_parser;
use mommy_lib::alu;
use mommy_lib::io;
use mommy_lib::loops;
use mommy_lib::conditions;
use mommy_lib::declaration;
use mommy_lib::errors::MommyErrorResponse;
use std::fs;
use std::collections::HashMap;
use std::env;
use std::process::Command;

fn parse_line(
    tokens: Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyErrorResponse> {

    if tokens.is_empty() {
        return Ok(String::new());
    }

    match tokens[0].as_str() {

        "mayihave" => declaration::may_i_have(&tokens, symbols),

        "add" => alu::calculate_two(&tokens[1], "+", &tokens[3], symbols),
        "divide" => alu::calculate_two(&tokens[1], "/", &tokens[3], symbols),
        "subtract" => alu::calculate_two(&tokens[1], "-", &tokens[3], symbols),
        "multiply" => alu::calculate_two(&tokens[1], "*", &tokens[3], symbols),

        "say" => io::say(&tokens, symbols),

        "punishme" => Ok(loops::punish_me(&tokens)),
        "done" => Ok(loops::done()),

        "ask" => Ok(conditions::ask(&tokens)),
        "or" => Ok(conditions::or()),

        "leave" => Ok("return 0;".to_string()),

        _ => Ok(format!("// Unknown command {}", tokens[0])),
    }
}



// This is too big, i will deal with this later
fn main() {
    // 1. GET ARGUMENTS & SETUP FILENAMES
    let args: Vec<String> = env::args().collect();

    let input_filename = if args.len() > 1 {
        &args[1]
    } else {
        "sandbox/template.mommy"
    };

    // Prepare the output names *before* we start writing
    // e.g., "sandbox/story.mommy" -> "sandbox/story.c" and "sandbox/story.exe"
    let c_filename = input_filename.replace(".mommy", ".c");
    let exe_filename = input_filename.replace(".mommy", ".exe");

    println!("Mommy is reading: {}", input_filename);

    // 2. READ THE FILE
    let content = fs::read_to_string(input_filename)
        .expect(&format!("Could not read file: {}", input_filename));

    // 3. CREATE THE C FILE (Use the dynamic name!)
    let mut output_file = fs::File::create(&c_filename)
        .expect("Could not create C file");

    let mut symbol_table: HashMap<String, String> = HashMap::new();

    // 4. WRITE THE C CODE
    // We write into 'output_file' which points to your dynamic .c file
    use std::io::Write; // Make sure we can use writeln!
    writeln!(output_file, "#include <stdio.h>").unwrap();
    writeln!(output_file, "int main(){{").unwrap();

    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        // Assuming these functions come from your mommy_lib
        let tokens = syntax_parser::insert_token(trimmed_line);
        let result = parse_line(tokens, &mut symbol_table);

        match result {
            Ok(c_code) => {
                writeln!(output_file, "    {}", c_code).unwrap();
            }
            Err(e) => {
                eprintln!("COMPILATION ABORTED:\nLine: \"{}\"\nMommy says: {}", trimmed_line, e);
                // We stop here so we don't try to run broken code
                return;
            }
        }
    }

    writeln!(output_file, "}}").unwrap();

    // Crucial: Save the file before GCC tries to read it
    drop(output_file);

    // 5. RUN GCC DYNAMICALLY
    println!("Compiling {}...", c_filename);

    let output = Command::new("gcc")
        .arg(&c_filename)   // Compile the dynamic .c file
        .arg("-o")
        .arg(&exe_filename) // Output the dynamic .exe
        .output()
        .expect("Failed to execute GCC");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    // 6. RUN THE PROGRAM
    println!("Running {}...\n", exe_filename);
    println!("--- MOMMY OUTPUT BEGINS ---");

    let run_path = if exe_filename.contains('/') || exe_filename.contains('\\') {
        exe_filename // It already has a path (e.g., "sandbox/test.exe")
    } else {
        format!("./{}", exe_filename) // It needs a path (e.g., "./test.exe")
    };

    let _ = Command::new(&run_path)
        .status()
        .expect("Failed to run the executable.");

    println!("\n--- MOMMY OUTPUT ENDS ---");
}