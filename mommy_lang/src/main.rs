use mommy_lib::syntax_parser;
use mommy_lib::alu;
use mommy_lib::io;
use mommy_lib::loops;
use mommy_lib::conditions;
use mommy_lib::declaration;
use mommy_lib::errors::MommyErrorResponse;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::collections::HashMap;



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


fn main(){
    let content = fs::read_to_string("sandbox/test.mommy").expect("Could not read the file Does test Mommy exist?");
    let mut symbol_table: HashMap<String, String> = HashMap::new();

    let mut output_file = fs::File::create("sandbox/test.c").expect("Could not create file");

    writeln!(output_file, "#include <stdio.h>").unwrap();
    writeln!(output_file, "int main(){{").unwrap();

    for line in content.lines(){
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        let tokens = syntax_parser::insert_token(trimmed_line);
        let result = parse_line(tokens, &mut symbol_table);
        match result {
            Ok(c_code) => {
                // Happy Path: Write the C code to the file
                writeln!(output_file, "    {}", c_code).unwrap();
            }
            Err(e) => {
                // Sad Path: Print your toxic error message and STOP compiling
                eprintln!("COMPILATION ABORTED:\nLine: \"{}\"\nMommy says: {}", trimmed_line, e);
                return; // Exit the program immediately
            }
        }

    }

    writeln!(output_file, "}}").unwrap();
    drop(output_file);
    println!("Transpilation successful. Now summoning GCC...");


    println!("Compiling...");

    let output = Command::new("gcc")
        .arg("sandbox/test.c")
        .arg("-o")
        .arg("sandbox/test.exe")
        .output()
        .expect("Failed to execute GCC");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    println!("GCC finished. Running the program...\n");
    println!("--- MOMMY OUTPUT BEGINS ---");

    let _ = Command::new("./sandbox/test.exe")
        .status()
        .expect("Failed to run the executable.");

    println!("\n--- MOMMY OUTPUT ENDS ---");
}