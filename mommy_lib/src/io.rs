//!
//! This is the io crate of mommylang.
//!
//! Language syntax:
//! 1. SAY
//! - String Literal: "say "string_literal"
//! - Array Access: "say array_name in index"
//! - Array wildcard for ASCII: "say array_name in ?"
//! - Scalar Variable: "say variable_name_or_literal"
//! 2. LISTEN
//! -  Listen to variable input: "listen var_name"
//! -  Listen with size limit: "listen var_name upto size"
//!
//! Notes:
//! 1. Float, integer, and ASCII (unique int for string) types are supported.
//! 2. For "say", if the variable is undeclared or type-mismatched, an error is returned.
//! 3. For "listen", if the variable is undeclared or not a supported type, an error is returned.
//!
//!
//!
//!
use std::collections::HashMap;
use crate::responses::MommyLangError;
use crate::constants;
use crate::validate_syntax;

pub fn say(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {

    if validate_syntax::is_missing_say_args(tokens.len()) {
        return Err(MommyLangError::MissingArguments);
    }

    let message = &tokens[constants::IDX_IO_VALUE];

    if message.starts_with("\"") {
        return say_literal(tokens);
    }

    if tokens.len() >= constants::ARGS_MIN_IO_ARRAY && tokens[constants::IDX_IO_KEY_IN] == constants::KW_IN {
        return say_array(tokens, symbols);
    }

    say_scalar(tokens, symbols)
}


fn say_literal(tokens: &Vec<String>) -> Result<String, MommyLangError> {
    // Syntax: say "string_literal"
    let full_msg = tokens[constants::IDX_IO_VALUE..].join(constants::SYM_WHITESPACE);
    let clean_msg = full_msg.trim_matches('"');
    Ok(format!("printf(\"{}\\n\");", clean_msg))
}



fn say_array(
    tokens: &Vec<String>,
    symbols: &HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: "say array_name in index"
    let name = &tokens[constants::IDX_IO_VALUE];
    let index = &tokens[constants::IDX_IO_ARR_IDX];

    let array_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    if validate_syntax::is_type_mismatch(array_type) {
        return Err(MommyLangError::TypeMismatch);
    }

    let parts: Vec<&str> = array_type.split(constants::SYM_SPLITTER).collect();


    let inner_type = validate_syntax::select_inner_type(parts.len(), parts[constants::IDX_META_DATA_TYPE]);
    let max_size = validate_syntax::select_max_size(parts.len(), parts[constants::IDX_META_SIZE]);

    if max_size == 0{
        return Err(MommyLangError::AccessViolation);
    }

    if validate_syntax::is_kw_all(&index){
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
        if (array_type.starts_with(constants::KW_ARRAY) || array_type.starts_with(constants::KW_HEAP)) &&
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
        constants::TYPE_FLOAT => Ok(format!("printf(\"%f\\n\", {}[{}]);", name, index)),
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

    if let Ok(_) = name.parse::<i32>() {
        return Ok(format!("printf(\"%d\\n\", {});", name));
    }

    if let Ok(_) = name.parse::<f64>() {
        return Ok(format!("printf(\"%f\\n\", {});", name));
    }

    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    match var_type.as_str() {
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

    if validate_syntax::is_missing_say_args(tokens.len()) {
        return Err(MommyLangError::MissingArguments);
    }

    let name = &tokens[constants::IDX_IO_VALUE];
    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    let buffer_size = if tokens.len() >= constants::ARGS_MIN_IO_ARRAY && tokens[2] == constants::KW_UPTO {
        tokens[3].clone()
    } else {
        get_size_from_type(var_type, "128")
    };

    match var_type.as_str() {
        t if t == constants::TYPE_INT => {
            Ok(format!(
                "{{ char _mommy_buf[64]; if(fgets(_mommy_buf, 64, stdin)) {{ {} = atoi(_mommy_buf); }} }}",
                name
            ))
        },

        t if t == constants::TYPE_FLOAT => {
            Ok(format!(
                "{{ char _mommy_buf[64]; if(fgets(_mommy_buf, 64, stdin)) {{ {} = atof(_mommy_buf); }} }}",
                name
            ))
        },

        t if t.contains(constants::TYPE_STRING) || t.contains(constants::C_TYPE_CHAR_PTR) => {
             Ok(format!(
                "fgets({}, {}, stdin); {}[strcspn({}, \"\\n\")] = 0;",
                name, buffer_size, name, name
            ))
        },

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

fn get_size_from_type(type_str: &str, default: &str) -> String {
    let parts: Vec<&str> = type_str.split(constants::SYM_SPLITTER).collect();
    if parts.len() > 2 {
        parts[2].to_string()
    } else {
        default.to_string()
    }
}
