//! # MommyLang Compiler Infrastructure
//!
//! This is the entry point for the MommyLang compiler. It handles file parsing,
//! C transpilation, and GCC invocation.
//!
//! ---
//!
//! ## 🛠️ Development Roadmap
//!
//! ### Phase 2: The Discipline Update (Current)
//! Focused on memory safety, data structures, and stricter control.
//! - [x] **Step 0:** Refactor magic numbers to `constants` module.
//! - [x] **Step 1:** Data Structures - Stack Arrays (`group`).
//! - [ ] **Step 2:** Memory Management - Heap Allocation (`ibegyou`). **[CURRENT WORK]**
//! - [ ] **Step 3:** Input Handling - Stdin Wrapper (`listen`).
//! - [ ] **Step 4:** Package System - Modules (`please use`).
//! - [ ] **Step 5:** Security - Permissions & Sandboxing.
//!
//! ### Phase 3: The Stockholm Update
//! Focused on OS-level features and system dependency.
//! - [ ] **Step 1:** MommyOS (Kernel/Process Management).
//! - [ ] **Step 2:** Cleanup (Refactoring & Optimization).
//!
//! ### Bonus Objectives
//! - [ ] **Mommy's Fingers:** Registry-like assembly manipulation.
//!
//! ---
//!
//! ## 🧠 Psychological Phases (The Lore)
//!
//! The compiler's personality evolves with the user's proficiency.
//!
//! 1.  **Phase 1 (Abusive):** Rejection. *"You are stupid."* (Syntax Errors = Insults)
//! 2.  **Phase 2 (Discipline):** Correction. *"Do it my way."* (Strict Typing/Borrow Checking)
//! 3.  **Phase 3 (Stockholm):** Acceptance. *"This is my home."* (Vendor Lock-in)
//!
//! ### Future Expansions
//! * **Phase 3.5 (Gaslighting):** Confusion. *"Did I do that?"* (Randomized warnings)
//! * **Phase 4 (Domestic):** Responsibility. *"I must feed the system."* (Manual memory management)
//! * **Phase 5 (Freedom):** False Hope. *"I can leave... but do I want to?"* (The final test)


use std::io::Write;
use mommy_lib::syntax_lexer;
use mommy_lib::alu;
use mommy_lib::io;
use mommy_lib::loops;
use mommy_lib::conditions;
use mommy_lib::declaration;
use std::collections::HashMap;
use std::{env, fs};
use std::process::Command;
use mommy_lib::responses;
use mommy_lib::constants;
use mommy_lib::lang_enums::ScopeType;
use mommy_lib::packages;
use mommy_lib::shell_format::{print_line, eprint_line};



fn parse_line(
    tokens: Vec<String>,
    symbols: &mut HashMap<String, String>,
    scope_stack: &mut Vec<ScopeType>
) -> Result<String, responses::MommyLangError> {



    if tokens.is_empty() {
        return Ok(String::new());
    }

    let command = mommy_lib::lang_syntax::MommyLangSyntax::from_str(&tokens[0]);

    match command {

        // --- Variables ---
        mommy_lib::lang_syntax::MommyLangSyntax::Declaration => {
            declaration::create_variable(&tokens, symbols)
        },
        mommy_lib::lang_syntax::MommyLangSyntax::Assignment => {
            declaration::replace(&tokens, symbols)
        },

        mommy_lib::lang_syntax::MommyLangSyntax::Array =>{
            declaration::create_array(&tokens, symbols)
        }
        
        mommy_lib::lang_syntax::MommyLangSyntax::String =>{ // new
            declaration::create_array(&tokens, symbols)
        }

        // --- Math (ALU) ---
        mommy_lib::lang_syntax::MommyLangSyntax::Math => {
            if tokens.len() < constants::ARGS_MIN_MATH {
                return Err(responses::MommyLangError::MissingArguments);
            }

            let operand = match tokens[0].as_str() {
                "add" => constants::C_OP_ADD,
                "divide" => constants::C_OP_DIV,
                "subtract" => constants::C_OP_SUB,
                "multiply" => constants::C_OP_MUL,
                "mod" => constants::C_OP_MOD,
                _ => return Err(responses::MommyLangError::SyntaxError), // Should never happen
            };

            alu::calculate_two(&tokens[constants::IDX_MATH_TARGET], operand, &tokens[constants::IDX_MATH_SOURCE], symbols)
        },

        // --- I/O ---
        mommy_lib::lang_syntax::MommyLangSyntax::IO => {
            io::say(&tokens, symbols)
        },

        // --- Loops ---
        mommy_lib::lang_syntax::MommyLangSyntax::LoopStartBasic => { // "punishme"
            scope_stack.push(ScopeType::Loop);
            Ok(loops::for_loop(&tokens))
        }
        mommy_lib::lang_syntax::MommyLangSyntax::LoopEnd => {   // "done"
            match scope_stack.pop() {
                Some(_) => Ok(loops::done()), // "}"
                None => Err(responses::MommyLangError::UnexpectedDone)
            }
        },
        mommy_lib::lang_syntax::MommyLangSyntax::LoopBreak => { // "satisfied"
            if !scope_stack.contains(&ScopeType::Loop) {
                return Err(responses::MommyLangError::UnexpectedSatisfied);
            }
            Ok(loops::satisfied())
        },

        mommy_lib::lang_syntax::MommyLangSyntax::LoopStartCondition => { // "punishmeif"
            if tokens.len() < 2 {
                return Err(responses::MommyLangError::MissingArguments);
            }
            scope_stack.push(ScopeType::Loop);
            Ok(loops::while_loop(&tokens))
        }

        // --- Conditions ---
        mommy_lib::lang_syntax::MommyLangSyntax::Condition => { // "ask"
            scope_stack.push(ScopeType::Condition);
            conditions::ask(&tokens)
        },
        mommy_lib::lang_syntax::MommyLangSyntax::ConditionElse => { // "or"
            match scope_stack.last() {
                Some(ScopeType::Condition) => {
                    conditions::or()
                },
                Some(ScopeType::Loop) => {
                    // You can't put 'or' directly inside a loop without an 'ask'
                    Err(responses::MommyLangError::OrphanElse)
                },
                None => {
                    // You can't put 'or' with nothing before it
                    Err(responses::MommyLangError::OrphanElse)
                }
                _ => {
                    Err(responses::MommyLangError::SyntaxError)
                }
            }
        },

        // --- System ---
        mommy_lib::lang_syntax::MommyLangSyntax::ProgramEnd => { // "leave"
            Ok(constants::C_EXIT_SUCC.to_string())
        },

        // --- Error Handling ---
        mommy_lib::lang_syntax::MommyLangSyntax::Unknown => {
            Err(responses::MommyLangError::SyntaxError)
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

        if args.len() < constants::ARGS_MIN_FILE{
            return Err(responses::MommyLangError::StatusNoFile.to_string())
        }

        let input_path = args[constants::IDX_FILE_NAME].clone();

        if !input_path.ends_with(constants::EXT_SOURCE){
            return Err(responses::MommyLangError::WrongFileType.to_string())
        }

        let c_path = input_path.replace(constants::EXT_SOURCE, constants::EXT_C);
        let exe_path = input_path.replace(constants::EXT_SOURCE, constants::EXT_EXE);

        Ok(Config{
            input_path,
            c_path,
            exe_path,
        })
    }
}

fn transpile_code_to_c(config: &Config) -> Result<(), String> {

    let mut scope_stack: Vec<ScopeType> = Vec::new();

    let content = fs::read_to_string(&config.input_path)
        .map_err(|_| format!("{} :{}", responses::MommyLangError::CannotReadFile, config.input_path))?;

    let mut output_file = fs::File::create(&config.c_path)
        .map_err(|_| responses::MommyLangError::CannotCreateCFile.to_string())?;

    let mut symbol_table: HashMap<String, String> = HashMap::new();

    // this should be dynamic as we want to make the user add modules/packages
    let include = packages::CStandardPackages::InputOutput.to_string();
    if include.trim().is_empty() {
        eprint_line(responses::MommyLangError::UnknownPackage);
    } else {
        writeln!(output_file, "{}", include).unwrap();
    }


    writeln!(output_file, "{}", constants::C_MAIN_START).unwrap(); // int main

    for (i, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let tokens = syntax_lexer::insert_token(trimmed_line);
        let result = parse_line(tokens, &mut symbol_table, &mut scope_stack);

        match result {
            Ok(c_code) => {
                writeln!(output_file, "    {}", c_code).unwrap();
            }
            Err(e) => {
                // Return the error so main stops!
                return Err(format!("{}, {}: {}", constants::MSG_ERR_LINE
                                   , i + 1, e));
            }
        }
    }

    if !scope_stack.is_empty() {
        return Err(responses::MommyLangError::UnclosedBlock.to_string());
    }

    writeln!(output_file, "{}", constants::C_MAIN_END).unwrap();

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();


    let config = match Config::new(&args){ // Prepare the file
        Ok(cfg) => cfg,
        Err(e) => {
            eprint_line(e);
            std::process::exit(1);
        }
    };


    if let Err(e) = transpile_code_to_c(&config){ //Convert mommylang to C
        print_line(responses::MommyLangError::ErrorBegins);
        eprint_line(e);
        show_c_conversion_error(&config); // show fragmented c code
        eprint_line(responses::MommyLangError::ConvertLangFailed);
        print_line(responses::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }

    if let Err(e) = compile_to_gcc(&config){ //use GCC to create exe file for the converted C
        print_line(responses::MommyLangError::ErrorBegins);
        eprint_line(responses::MommyLangError::GCCError);
        eprint_line(e);
        print_line(responses::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }

    print_line(responses::MommyLangStatus::CodeOutputBegins);
    if let Err(e) = run_mommy_file(&config){ // Run the exe file
        print_line(responses::MommyLangError::ErrorBegins);
        eprint_line(responses::MommyLangError::RuntimeError);
        eprint_line(e);
        print_line(responses::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }
    print_line(responses::MommyLangStatus::CodeOutputEnds);
}

fn run_mommy_file(config: &Config) -> Result<(), String> {
    let output = if config.exe_path.contains(constants::SYM_SLASH)
        || config.exe_path.contains(constants::SYM_BACKSLASH){
        config.exe_path.clone()
    } else {
        format!("{} {}", constants::PATH_DEFAULT, config.exe_path)
    };

    let status = Command::new(output)
        .status()
        .map_err(|_| responses::MommyLangError::ExecutableFile.to_string())?;

    if !status.success() {
        return Err(format!("{} {}",responses::MommyLangError::RunFile, status.code().unwrap_or(-1)));
    }

    Ok(())
}
fn compile_to_gcc(config: &Config) -> Result<(), String>{

    let output = Command::new(constants::CMD_GCC)
        .arg(&config.c_path)
        .arg(constants::CMD_GCC_FLAG)
        .arg(&config.exe_path)
        .output()
        .map_err(|_| responses::MommyLangError::GCCNotFound.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(error_msg)
    }

}

fn show_c_conversion_error(log: &Config){
    let contents = fs::read_to_string(&log.c_path).expect(&responses::MommyLangError::CannotReadFile.to_string()); // temporary, replace it with legit error
    print_line(responses::MommyLangStatus::ConversionErrorStart);
    println!("{}", contents);
    print_line(responses::MommyLangStatus::ConversionErrorEnds);
    let _ = fs::remove_file(&log.c_path); // Remove the file if the process of compiling it into C fails. Deleting the file manually is tiring.
}
