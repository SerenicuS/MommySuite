use std::collections::HashMap;
use crate::responses::MommyLangError;
use crate::constants;

pub fn say(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {

    if tokens.len() < constants::ARGS_MIN_IO {
        return Err(MommyLangError::MissingArguments)
    }

    // WAS: tokens[1]
    let message = &tokens[constants::IDX_IO_VALUE];

    if message.starts_with("\"") {
        return say_literal(tokens);
    }


    // WAS: tokens.len() >= 4
    if tokens.len() >= constants::ARGS_MIN_IO_ARRAY
        && tokens[constants::IDX_IO_KEY_IN] == constants::KW_IN {
        return say_array(tokens, symbols);
    }

    say_scalar(tokens, symbols)
}

fn say_literal(tokens: &Vec<String>) -> Result<String, MommyLangError> {
    // We join from IDX_IO_VALUE onwards to catch spaces in strings
    let full_msg = tokens[constants::IDX_IO_VALUE..].join(constants::SYM_WHITESPACE);
    let clean_msg = full_msg.trim_matches('"');
    Ok(format!("printf(\"{}\\n\");", clean_msg))
}

fn say_array(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {

    // WAS: tokens[1] and tokens[3]
    let name = &tokens[constants::IDX_IO_VALUE];
    let index = &tokens[constants::IDX_IO_ARR_IDX];

    let array_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    if !array_type.starts_with(constants::KW_ARRAY) &&
        array_type != constants::TYPE_STRING &&
        array_type != constants::KW_POINTER {
        return Err(MommyLangError::TypeMismatch);
    }

    let parts: Vec<&str> = array_type.split(constants::SYM_SPLITTER).collect();
    let inner_type = if parts.len() > 1 { parts[1] } else { constants::TYPE_STRING };
    let max_size = if parts.len() > 2 { parts[2].parse::<usize>().unwrap_or(0) } else { 0 };

    if index == constants::KW_ALL {
        if max_size == 0 { return Err(MommyLangError::AccessViolation); }
        return match inner_type {
            t if t == constants::TYPE_ASCII =>
                Ok(format!("for (int i = 0; i < {}; i++) {{ printf(\"%c\", {}[i]); }} printf(\"\\n\");", max_size, name)),
            t if t == constants::TYPE_FLOAT =>
                Ok(format!("for (int i = 0; i < {}; i++) {{ printf(\"%f \", {}[i]); }} printf(\"\\n\");", max_size, name)),
            t if t == constants::C_TYPE_CHAR_PTR || t == constants::TYPE_STRING =>
                Ok(format!("for (int i = 0; i < {}; i++) {{ printf(\"%s \", {}[i]); }} printf(\"\\n\");", max_size, name)),
            _ => Ok(format!("for (int i = 0; i < {}; i++) {{ printf(\"%d \", {}[i]); }} printf(\"\\n\");", max_size, name)),
        };
    }

    if let Ok(idx_num) = index.parse::<usize>() {
        if array_type.starts_with(constants::KW_ARRAY) &&
           idx_num >= max_size &&
           max_size != constants::SIZE_UNKNOWN
        {
            return Err(MommyLangError::AccessViolation);
        }
    }

    else if symbols.contains_key(index) {

    }
    else {
        // It's garbage (like "say arr in $#@")
        return Err(MommyLangError::SyntaxError);
    }

    match inner_type {
        "float" => Ok(format!("printf(\"%f\\n\", {}[{}]);", name, index)),
        t if t == constants::C_TYPE_CHAR_PTR || t == constants::TYPE_STRING =>
            Ok(format!("printf(\"%s\\n\", {}[{}]);", name, index)),
        t if t == constants::TYPE_ASCII =>
            Ok(format!("printf(\"%c\\n\", {}[{}]);", name, index)),
        _ => Ok(format!("printf(\"%d\\n\", {}[{}]);", name, index)),
    }
}

fn say_scalar(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {

    let name = &tokens[constants::IDX_IO_VALUE];
    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    match var_type.as_str() {
        // FIX: Add Float Support here
        t if t == constants::TYPE_FLOAT =>
            Ok(format!("printf(\"%f\\n\", {});", name)),

        t if t == constants::TYPE_STRING || t == constants::C_TYPE_CHAR_PTR =>
            Ok(format!("printf(\"%s\\n\", {});", name)),

        t if t == constants::TYPE_ASCII =>
            Ok(format!("printf(\"%c\\n\", {});", name)),

        t if t == constants::KW_POINTER =>
            Ok(format!("if ({0} == NULL) {{ printf(\"NULL\\n\"); }} else {{ printf(\"%d\\n\", *{0}); }}", name)),

        _ => Ok(format!("printf(\"%d\\n\", {});", name)),
    }
}

pub fn listen(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {

    // Syntax: listen <var> [upto <size>]
    if tokens.len() < constants::ARGS_MIN_IO {
        return Err(MommyLangError::MissingArguments);
    }

    let name = &tokens[constants::IDX_IO_VALUE]; // tokens[1]
    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    // 1. Determine the "Safety Limit" (Buffer Size)
    // If they said "upto 50", use 50. Otherwise, try to find the array size.
    let buffer_size = if tokens.len() >= constants::ARGS_MIN_IO_ARRAY && tokens[2] == constants::KW_UPTO {
        tokens[3].clone()
    } else {
        // Helper: Extract size from "array|ascii|6" or default to 128
        get_size_from_type(var_type, "128")
    };

    // 2. Generate C Code based on the variable type
    match var_type.as_str() {
        // CASE: Integer (mayihave 0 in age as int)
        // Logic: Read into temp buffer -> atoi -> store
        t if t == constants::TYPE_INT => {
            Ok(format!(
                "{{ char _mommy_buf[64]; if(fgets(_mommy_buf, 64, stdin)) {{ {} = atoi(_mommy_buf); }} }}",
                name
            ))
        },

        // CASE: Float (Bonus support)
        t if t == constants::TYPE_FLOAT => {
            Ok(format!(
                "{{ char _mommy_buf[64]; if(fgets(_mommy_buf, 64, stdin)) {{ {} = atof(_mommy_buf); }} }}",
                name
            ))
        },

        // CASE: String / Char Array (mayihave " " in name as string)
        // Logic: Standard fgets + strip newline
        t if t.contains(constants::TYPE_STRING) || t.contains(constants::C_TYPE_CHAR_PTR) => {
             Ok(format!(
                "fgets({}, {}, stdin); {}[strcspn({}, \"\\n\")] = 0;",
                name, buffer_size, name, name
            ))
        },

        // CASE: ASCII Array (group 6 in secret as ascii)
        // Logic: Read to temp char buffer -> Loop -> Cast to int -> Store in int array
        t if t.contains(constants::TYPE_ASCII) => {
            Ok(format!(
                "{{ char _temp_ascii[{}]; \
                   if(fgets(_temp_ascii, {}, stdin)) {{ \
                       for(int i=0; i<{}; i++) {{ \
                           if(_temp_ascii[i] == '\\0' || _temp_ascii[i] == '\\n') {{ \
                               {}[i] = 0; \
                               for(int j=i+1; j<{}; j++) {{ {}[j] = 0; }} \
                               break; \
                           }} \
                           {}[i] = (int)_temp_ascii[i]; \
                       }} \
                   }} \
                }}",
                buffer_size, buffer_size, buffer_size, name, buffer_size, name, name
            ))
        },

        _ => Err(MommyLangError::TypeMismatch)
    }
}

// Helper to grab size from "array|type|size" string
fn get_size_from_type(type_str: &str, default: &str) -> String {
    let parts: Vec<&str> = type_str.split(constants::SYM_SPLITTER).collect();
    if parts.len() > 2 {
        parts[2].to_string()
    } else {
        default.to_string()
    }
}