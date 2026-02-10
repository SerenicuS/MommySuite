use std::collections::HashMap;
use crate::responses::MommyLangError;
use crate::constants;
// ================================================================
// THE ROUTER (Traffic Cop)
// ================================================================

pub fn say(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {

    if tokens.len() < constants::MIN_ARGS_LEN {
        return Err(MommyLangError::MissingArguments)
    }

    let message = &tokens[1];

    // 1. Is it a String Literal? ("Hello")
    if message.starts_with("\"") {
        return say_literal(tokens);
    }

    // 2. Is it an Array Access? (scores in 0)
    // tokens: [0] [1]    [2] [3]
    if tokens.len() >= 4 && tokens[2] == "in" {
        return say_array(tokens, symbols);
    }

    // 3. It must be a normal Variable (x)
    say_scalar(tokens, symbols)
}

// ================================================================
// SPECIALIZED WORKERS
// ================================================================

fn say_literal(tokens: &Vec<String>) -> Result<String, MommyLangError> {
    // Reconstruct the full string (e.g., "Hello World")
    let full_msg = tokens[1..].join(" ");
    Ok(format!("printf(\"%s\\n\", {});", full_msg))
}

fn say_array(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {
    let name = &tokens[1];
    let index = &tokens[3];

    // Get metadata "array:int:5"
    let meta_data = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    if !meta_data.starts_with("array") {
        return Err(MommyLangError::TypeMismatch);
    }

    // Bounds Check (Optional but good)
    let parts: Vec<&str> = meta_data.split(':').collect();
    let inner_type = parts[1];
    let max_size = parts[2].parse::<usize>().unwrap();

    // Verify index if it is a number
    if let Ok(idx_num) = index.parse::<usize>() {
        if idx_num >= max_size {
            return Err(MommyLangError::AccessViolation);
        }
    }

    // Generate Printf based on type
    match inner_type {
        "float" => Ok(format!("printf(\"%f\\n\", {}[{}]);", name, index)),
        "char*" | "String" => Ok(format!("printf(\"%s\\n\", {}[{}]);", name, index)),
        _ => Ok(format!("printf(\"%d\\n\", {}[{}]);", name, index)), // Default to int
    }
}

fn say_scalar(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {
    let name = &tokens[1];
    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    if var_type == "String" {
        Ok(format!("printf(\"%s\\n\", {});", name))
    } else if var_type == "float" {
        Ok(format!("printf(\"%f\\n\", {});", name))
    } else if var_type == "pointer" {
        Ok(format!("printf(\"%p\\n\", (void*){});", name))
    } else {
        Ok(format!("printf(\"%d\\n\", {});", name))
    }
}