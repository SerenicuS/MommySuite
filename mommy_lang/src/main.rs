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

fn parse_line(
    tokens: Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, mommy_response::MommyLangErrorResponse> {

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


struct Config{
    input_path: String,
    c_path: String,
    exe_path: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, String>{
        if args.len() < 2{
            return Err(mommy_response::MommyLangErrorResponse::StatusNoFile.to_string())
        }

        let input_path = args[1].clone();

        if !input_path.ends_with(".mommy"){
            return Err(mommy_response::MommyLangErrorResponse::WrongFileType.to_string())
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

    let content = fs::read_to_string(&config.input_path)
        .map_err(|_| format!("Could not read file: {}", config.input_path))?;

    let mut output_file = fs::File::create(&config.c_path)
        .map_err(|_| "Could not create C file")?;

    let mut symbol_table: HashMap<String, String> = HashMap::new();

    writeln!(output_file, "#include <stdio.h>").unwrap();
    writeln!(output_file, "int main(){{").unwrap();

    for (i, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let tokens = syntax_parser::insert_token(trimmed_line);
        let result = parse_line(tokens, &mut symbol_table);

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

    println!("{}", mommy_response::MommyLangGeneralResponse::ReadingFile);


    if let Err(e) = transpile_code_to_c(&config){ //Convert mommylang to C
        eprintln!("{}", mommy_response::MommyLangErrorResponse::ConvertLangFailed);
        eprintln!("{}", e);
        std::process::exit(1);
    }

    if let Err(e) = compile_to_gcc(&config){ //use GCC to create exe file for the converted C
        eprintln!("{}", mommy_response::MommyLangErrorResponse::TranspilingError);
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("--- MOMMY OUTPUT BEGINS ---");

    if let Err(e) = run_mommy_file(&config){ // Run the exe file
        eprintln!("{}", mommy_response::MommyLangErrorResponse::RuntimeError);
        eprintln!("{}", e);
        std::process::exit(1);
    }
    println!("\n--- MOMMY OUTPUT ENDS ---");
}

fn run_mommy_file(config: &Config) -> Result<(), String> {
    let output = if config.exe_path.contains('/') || config.exe_path.contains('\\'){
        config.exe_path.clone()
    }
    else{
        format!("./{}", config.exe_path)
    };

    let _ = Command::new(output).status().map_err(|_| "Could not start the executable. Permission denied?".to_string())?;

    Ok(())
}

fn compile_to_gcc(config: &Config) -> Result<(), String>{

    let output = Command::new("gcc")
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