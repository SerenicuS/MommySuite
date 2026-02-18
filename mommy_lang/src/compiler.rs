use std::collections::HashMap;
use std::fs;
use std::io::Write;

use mommy_lib::alu;
use mommy_lib::conditions;
use mommy_lib::constants;
use mommy_lib::declaration;
use mommy_lib::io;
use mommy_lib::lang_enums::ScopeType;
use mommy_lib::loops;
use mommy_lib::package;
use mommy_lib::responses;
use mommy_lib::syntax_lexer;
use mommy_lib::shell_format::print_line;

use crate::config::Config;

fn parse_line(
    tokens: Vec<String>,
    symbols: &mut HashMap<String, String>,
    scope_stack: &mut Vec<ScopeType>,
    includes: &mut Vec<String>,
) -> Result<String, responses::MommyLangError> {
    if tokens.is_empty() {
        return Ok(String::new());
    }

    let command = mommy_lib::lang_syntax::MommyLangSyntax::from_str(&tokens[0]);

    match command {
        // --- Variables ---
        mommy_lib::lang_syntax::MommyLangSyntax::Declaration => {
            declaration::create_variable(&tokens, symbols)
        }
        mommy_lib::lang_syntax::MommyLangSyntax::Assignment => declaration::replace(&tokens, symbols),
        mommy_lib::lang_syntax::MommyLangSyntax::Array => declaration::create_array(&tokens, symbols),
        mommy_lib::lang_syntax::MommyLangSyntax::String => declaration::create_array(&tokens, symbols),

        // --- Dynamic Memory Allocation ---
        mommy_lib::lang_syntax::MommyLangSyntax::Malloc => declaration::allocate_heap(&tokens, symbols),
        mommy_lib::lang_syntax::MommyLangSyntax::FreeMalloc => declaration::deallocate_heap(&tokens, symbols),

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
                _ => return Err(responses::MommyLangError::SyntaxError),
            };

            alu::calculate_two(
                &tokens[constants::IDX_MATH_TARGET],
                operand,
                &tokens[constants::IDX_MATH_SOURCE],
                symbols,
            )
        }

        // --- I/O ---
        mommy_lib::lang_syntax::MommyLangSyntax::IO => io::say(&tokens, symbols),
        mommy_lib::lang_syntax::MommyLangSyntax::ReadInput => io::listen(&tokens, symbols),

        // --- Loops ---
        mommy_lib::lang_syntax::MommyLangSyntax::LoopStartBasic => {
            scope_stack.push(ScopeType::Loop);
            Ok(loops::for_loop(&tokens))
        }
        mommy_lib::lang_syntax::MommyLangSyntax::LoopEnd => match scope_stack.pop() {
            Some(_) => Ok(loops::done()),
            None => Err(responses::MommyLangError::UnexpectedDone),
        },
        mommy_lib::lang_syntax::MommyLangSyntax::LoopBreak => {
            if !scope_stack.contains(&ScopeType::Loop) {
                return Err(responses::MommyLangError::UnexpectedSatisfied);
            }
            Ok(loops::satisfied())
        }
        mommy_lib::lang_syntax::MommyLangSyntax::LoopStartCondition => {
            if tokens.len() < 2 {
                return Err(responses::MommyLangError::MissingArguments);
            }
            scope_stack.push(ScopeType::Loop);
            Ok(loops::while_loop(&tokens))
        }

        // --- Conditions ---
        mommy_lib::lang_syntax::MommyLangSyntax::Condition => {
            scope_stack.push(ScopeType::Condition);
            conditions::ask(&tokens)
        }
        mommy_lib::lang_syntax::MommyLangSyntax::ConditionElse => match scope_stack.last() {
            Some(ScopeType::Condition) => conditions::or(),
            Some(ScopeType::Loop) | None => Err(responses::MommyLangError::OrphanElse),
            _ => Err(responses::MommyLangError::SyntaxError),
        },

        // --- System ---
        mommy_lib::lang_syntax::MommyLangSyntax::ProgramEnd => Ok(constants::C_EXIT_SUCC.to_string()),
        mommy_lib::lang_syntax::MommyLangSyntax::IncludeLib => {
            if tokens.len() < 2 {
                return Err(responses::MommyLangError::MissingArguments);
            }
            let include = package::add_package(&tokens[1])?;
            if !include.is_empty() && !includes.contains(&include) {
                includes.push(include);
            }
            Ok(String::new())
        }

        // --- Error Handling ---
        mommy_lib::lang_syntax::MommyLangSyntax::Unknown => Err(responses::MommyLangError::SyntaxError),
    }
}

pub fn transpile_code_to_c(config: &Config) -> Result<(), String> {
    let mut scope_stack: Vec<ScopeType> = Vec::new();

    let content = fs::read_to_string(&config.input_path)
        .map_err(|_| format!("{} :{}", responses::MommyLangError::CannotReadFile, config.input_path))?;

    let mut output_file = fs::File::create(&config.c_path)
        .map_err(|_| responses::MommyLangError::CannotCreateCFile.to_string())?;

    let mut symbol_table: HashMap<String, String> = HashMap::new();
    let mut includes: Vec<String> = Vec::new();
    let mut body_lines: Vec<String> = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let tokens = syntax_lexer::insert_token(trimmed_line);
        let result = parse_line(tokens, &mut symbol_table, &mut scope_stack, &mut includes);

        match result {
            Ok(c_code) => {
                if !c_code.is_empty() {
                    body_lines.push(c_code);
                }
            }
            Err(e) => {
                return Err(format!("{}, {}: {}", constants::MSG_ERR_LINE, i + 1, e));
            }
        }
    }

    if !scope_stack.is_empty() {
        return Err(responses::MommyLangError::UnclosedBlock.to_string());
    }

    for include in &includes {
        writeln!(output_file, "{}", include).unwrap();
    }
    if !includes.is_empty() {
        writeln!(output_file).unwrap();
    }

    writeln!(output_file, "{}", constants::C_MAIN_START).unwrap();

    for line in &body_lines {
        writeln!(output_file, "    {}", line).unwrap();
    }

    writeln!(output_file, "{}", constants::C_MAIN_END).unwrap();

    Ok(())
}

pub fn show_c_conversion_error(log: &Config) {
    let contents = fs::read_to_string(&log.c_path)
        .expect(&responses::MommyLangError::CannotReadFile.to_string());
    print_line(responses::MommyLangStatus::ConversionErrorStart);
    println!("{}", contents);
    print_line(responses::MommyLangStatus::ConversionErrorEnds);
    let _ = fs::remove_file(&log.c_path);
}
