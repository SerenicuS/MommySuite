use std::io::Write;
use mommy_lib::syntax_parser;
use mommy_lib::alu;
use mommy_lib::io;
use mommy_lib::loops;
use mommy_lib::conditions;
use mommy_lib::declaration;
use std::collections::HashMap;
use std::{env, fs};
use std::process::Command;
use mommy_lib::mommy_response;
use mommy_lib::constants;



/*
    TODO:
      PHASE 1 - DONE
      PHASE 2 - CURRENT
      0. Use Constants in every magic numbers in all files
      1. Data Structures - Arrays - group
      2. Memory Management - Malloc (Dynamic Memory) -ibegyou
      3. Input Handling - Scanf/Input commands -> listen
      4. Package System - Modules/Packages (The "please" keyword) -please(use) makemetalk(stdio.io?)
      5. Security - Permissions/Sandboxing features
      PHASE 3
      1. Operating System - MommyOS (Kernel/Process Management)
      2. Cleanup - Refactoring & Optimization

 */

fn parse_line(
    tokens: Vec<String>,
    symbols: &mut HashMap<String, String>,
    scope_depth: &mut i32
) -> Result<String, mommy_response::MommyLangError> {

    if tokens.is_empty() {
        return Ok(String::new());
    }

    let command = mommy_lib::commands::CommandType::from_str(&tokens[0]);

    match command {

        // --- Variables ---
        mommy_lib::commands::CommandType::Declaration => {
            declaration::create_variable(&tokens, symbols)
        },
        mommy_lib::commands::CommandType::Assignment => {
            declaration::replace_variable(&tokens, symbols)
        },

        // --- Math (ALU) ---
        mommy_lib::commands::CommandType::Math => {
            if tokens.len() < constants::MIN_ARGS_MATH {
                return Err(mommy_response::MommyLangError::MissingArguments);
            }

            let op = match tokens[0].as_str() {
                "add" => "+",
                "divide" => "/",
                "subtract" => "-",
                "multiply" => "*",
                _ => return Err(mommy_response::MommyLangError::SyntaxError), // Should never happen
            };

            alu::calculate_two(&tokens[1], op, &tokens[3], symbols)
        },

        // --- I/O ---
        mommy_lib::commands::CommandType::IO => {
            io::say(&tokens, symbols)
        },

        // --- Loops ---
        mommy_lib::commands::CommandType::LoopStart => { // "punishme"
            *scope_depth += 1;
            Ok(loops::for_loop(&tokens))
        },
        mommy_lib::commands::CommandType::LoopEnd => {   // "done"
            if *scope_depth == 0 {
                return Err(mommy_response::MommyLangError::UnexpectedDone);
            }
            *scope_depth -= 1;
            Ok(loops::done())
        },
        mommy_lib::commands::CommandType::LoopBreak => { // "satisfied"
            Ok(loops::satisfied())
        },

        // --- Conditions ---
        mommy_lib::commands::CommandType::Condition => { // "ask"
            *scope_depth += 1;
            conditions::ask(&tokens)
        },
        mommy_lib::commands::CommandType::ConditionElse => { // "or"
            conditions::or()
        },

        // --- System ---
        mommy_lib::commands::CommandType::ProgramEnd => { // "leave"
            Ok("return 0;".to_string())
        },

        // --- Error Handling ---
        mommy_lib::commands::CommandType::Unknown => {
            Err(mommy_response::MommyLangError::SyntaxError)
        }
    }
}


struct Config{
    input_path: String,
    c_path: String,
    exe_path: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, String>{

        if args.len() < constants::MIN_ARGS_FILE_CMD{
            return Err(mommy_response::MommyLangError::StatusNoFile.to_string())
        }

        let input_path = args[1].clone();

        if !input_path.ends_with(".mommy"){
            return Err(mommy_response::MommyLangError::WrongFileType.to_string())
        }

        let c_path = input_path.replace(".mommy", ".c");
        let exe_path = input_path.replace(".mommy", ".exe");

        Ok(Config{
            input_path,
            c_path,
            exe_path,
        })
    }
}

fn transpile_code_to_c(config: &Config) -> Result<(), String> {

    let mut scope_depth = 0;

    let content = fs::read_to_string(&config.input_path)
        .map_err(|_| format!("Could not read file: {}", config.input_path))?;

    let mut output_file = fs::File::create(&config.c_path)
        .map_err(|_| "Could not create C file")?;

    let mut symbol_table: HashMap<String, String> = HashMap::new();

    writeln!(output_file, "#include <stdio.h>").unwrap();
    writeln!(output_file, "#include <stdlib.h>").unwrap();
    writeln!(output_file, "int main(){{").unwrap();

    for (i, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let tokens = syntax_parser::insert_token(trimmed_line);
        let result = parse_line(tokens, &mut symbol_table, &mut scope_depth);

        match result {
            Ok(c_code) => {
                writeln!(output_file, "    {}", c_code).unwrap();
            }
            Err(e) => {
                // Return the error so main stops!
                return Err(format!("Line {}: {}", i + 1, e));
            }
        }
    }

    if scope_depth > 0 {
        return Err(mommy_response::MommyLangError::UnclosedBlock.to_string());
    }

    writeln!(output_file, "}}").unwrap();

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();


    let config = match Config::new(&args){ // Prepare the file
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };


    if let Err(e) = transpile_code_to_c(&config){ //Convert mommylang to C
        println!("{}", mommy_response::MommyLangError::ErrorBegins);
        eprintln!("{}", e);
        show_c_conversion_error(&config); // show fragmented c code
        eprintln!("{}", mommy_response::MommyLangError::ConvertLangFailed);
        println!("{}", mommy_response::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }

    if let Err(e) = compile_to_gcc(&config){ //use GCC to create exe file for the converted C
        println!("{}", mommy_response::MommyLangError::ErrorBegins);
        eprintln!("{}", mommy_response::MommyLangError::GCCError);
        eprintln!("{}", e);
        println!("{}", mommy_response::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }

    println!("--- MOMMY OUTPUT BEGINS ---");
    if let Err(e) = run_mommy_file(&config){ // Run the exe file
        println!("{}", mommy_response::MommyLangError::ErrorBegins);
        eprintln!("{}", mommy_response::MommyLangError::RuntimeError);
        eprintln!("{}", e);
        println!("{}", mommy_response::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }
    println!("\n--- MOMMY OUTPUT ENDS ---");
}

fn run_mommy_file(config: &Config) -> Result<(), String> {
    let output = if config.exe_path.contains('/') || config.exe_path.contains('\\'){
        config.exe_path.clone()
    } else {
        format!("./{}", config.exe_path)
    };

    let status = Command::new(output)
        .status()
        .map_err(|_| "Could not start the executable.".to_string())?;

    if !status.success() {
        return Err(format!("Mommy is disappointed. Program exited with code {}", status.code().unwrap_or(-1)));
    }

    Ok(())
}
fn compile_to_gcc(config: &Config) -> Result<(), String>{

    let output = Command::new(constants::COMPILER_TOOL)
        .arg(&config.c_path)
        .arg("-o")
        .arg(&config.exe_path)
        .output()
        .map_err(|_| "GCC not found. Is MinGW installed?")?;

    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(error_msg)
    }

}

fn show_c_conversion_error(log: &Config){
    let contents = fs::read_to_string(&log.c_path).expect(&mommy_response::MommyLangError::CannotReadFile.to_string()); // temporary, replace it with legit error
    println!("--- PARTIAL C CODE GENERATED ---");
    println!("{}", contents);
    println!("--- [CRASH HERE] ---");
    let _ = fs::remove_file(&log.c_path); // Remove the file if the process of compiling it into C fails. Deleting the file manually is tiring.
}