//!
//! This is the declaration crate of mommylang.
//!
//! Language syntax:
//! - Scalar variables: "mayihave value in var_name as type"
//! - Arrays: "group size in var_name as type"
//! - Heap allocation: "ibegyou size in var_name as type"
//! - Heap deallocation: "takeitback var_name"
//! - Reassignment: "replace var_name with new_value"
//! - Array write: "replace var_name in index with new_value"
//! - Array read: "replace var_name with array_name in index"
//! - Pointer address: "replace var_name with other_var address"
//! - Pointer dereference write: "replace var_name with new_value inside"
//!
//! Notes:
//! 1) Float, integer, and ASCII (unique int for string) types are supported.
//!
//! About the string implementation:
//! I avoid using char* directly because it is complex to manage at this stage.
//! Instead, strings are represented as arrays of ASCII integers. This keeps the
//! syntax simple for users while still letting the compiler emit valid C code.
//!
//!
//!
//!
//!


use std::collections::HashMap;
use crate::responses::MommyLangError;
use crate::constants;
use crate::validate_syntax;

// ================================================================
// PUBLIC FUNCTIONS (The Logic)
// ================================================================

pub fn create_variable(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    if  validate_syntax::is_missing_decl_args(tokens.len()){
        return Err(MommyLangError::MissingArguments);
    }

    let in_index = tokens.iter().position(|r| r == constants::KW_IN) // in
        .ok_or(MommyLangError::SyntaxError)?;

    let name_index = in_index + 1;
    let type_index = name_index + 2;

    if validate_syntax::is_name_and_index_greater_than_len(name_index, type_index, tokens.len()){
        return Err(MommyLangError::SyntaxError);
    }

    let name = &tokens[name_index];
    let raw_type = &tokens[type_index];

    validate_syntax::ensure_valid_name(name)?;
    validate_syntax::ensure_var_new(name, symbols)?;

    let c_type = get_c_type(raw_type);

    insert_symbol(raw_type, name, symbols);

    let value_tokens = &tokens[1..in_index];
    let mut value = value_tokens.join(constants::SYM_WHITESPACE);

    value = c_null(value.as_str());

    Ok(format!("{} {} = {};", c_type, name, value))
}



pub fn create_array(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    if validate_syntax::is_missing_decl_args(tokens.len()){
        return Err(MommyLangError::MissingArguments);
    }

    let size_str = &tokens[constants::IDX_DECL_VALUE];
    let name     = &tokens[constants::IDX_DECL_NAME];
    let raw_type = &tokens[constants::IDX_DECL_TYPE];

    validate_syntax::ensure_valid_name(name)?;
    validate_syntax::ensure_var_new(name, symbols)?;

    if validate_syntax::is_invalid_array_tokens(&tokens){
        return Err(MommyLangError::SyntaxError);
    }

    if validate_syntax::is_invalid_array_size(size_str.as_str()){
        return Err(MommyLangError::SyntaxError);
    }

    let meta = format!("{}:{}:{}", constants::KW_ARRAY, raw_type, size_str);
    symbols.insert(name.to_string(), meta);

    let c_type = match raw_type.as_str(){
        constants::TYPE_ASCII => constants::TYPE_INT,
        _ => get_c_type(&raw_type),
    };

    Ok(format!("{} {}[{}] = {{0}};", c_type, name, size_str))
}



/// Heap Allocation
pub fn allocate_heap(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: ibegyou <SIZE> in <NAME> as <TYPE>

    if validate_syntax::is_missing_decl_args(tokens.len()){
        return Err(MommyLangError::MissingArguments);
    }

    let size_val = &tokens[constants::IDX_DECL_VALUE];
    let name = &tokens[constants::IDX_DECL_NAME];
    let raw_type = &tokens[constants::IDX_DECL_TYPE];

    if validate_syntax::is_invalid_array_tokens(&tokens){
        return Err(MommyLangError::SyntaxError);
    }

    validate_syntax::ensure_valid_name(name)?;
    validate_syntax::ensure_var_new(name, symbols)?;

    let c_type = match raw_type.as_str() {
        constants::TYPE_ASCII => constants::TYPE_INT,
        _ => get_c_type(raw_type),
    };

    let meta = format!("heap:{}:{}", raw_type, size_val);
    symbols.insert(name.to_string(), meta);

    let c_code = format!(
        "{0}* {1} = ({0}*)malloc({2} * sizeof({0})); \
        if ({1} == NULL) {{ \
        printf(\"Mommy Error: No memory for {1}\\n\"); return 1; }}",
        c_type,
        name,
        size_val
    );


    Ok(c_code)
}

pub fn deallocate_heap( tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: takeitback <NAME>

    if validate_syntax::is_invalid_dealloc_tokens(&tokens){
        return Err(MommyLangError::MissingArguments);
    }

    let name = &tokens[constants::IDX_DECL_VALUE];

    validate_syntax::ensure_var_exists(name, symbols)?;

    Ok(format!("free({}); {} = NULL;", name, name))
}



pub fn replace(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    if tokens.len() < constants::ARGS_MIN_ASSIGN {
        return Err(MommyLangError::MissingArguments);
    }

    // Write
    if tokens[constants::IDX_ARR_KEY_IN] == constants::KW_IN {
        return replace_array_write(tokens, symbols);
    }

    // Read
    if tokens.len() >= constants::ARGS_MIN_ARR_ASSIGN
        && tokens[constants::IDX_ASSIGN_KEY_WITH] == constants::KW_WITH
        && tokens[constants::IDX_ARR_KEY_WITH] == constants::KW_IN {
        return replace_array_read(tokens, symbols);
    }

    // Scalar Value or Pointer
    if tokens[constants::IDX_ASSIGN_KEY_WITH] == constants::KW_WITH {
        return replace_scalar_value(tokens, symbols);
    }

    Err(MommyLangError::SyntaxError)
}

// ================================================================
// SPECIALIZED WORKERS
// ================================================================

fn replace_array_write(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: replace [NAME] in [IDX] with [VAL]
    if tokens.len() < constants::ARGS_MIN_ARR_ASSIGN || tokens[constants::IDX_ARR_KEY_WITH] != constants::KW_WITH {
        return Err(MommyLangError::SyntaxError);
    }

    let name  = &tokens[constants::IDX_ARR_NAME];
    let index = &tokens[constants::IDX_ARR_INDEX];
    let value = &tokens[constants::IDX_ARR_VALUE];

    validate_syntax::ensure_var_exists(name, symbols)?;
    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    if validate_syntax::is_type_mismatch(var_type.as_str()){
        return Err(MommyLangError::TypeMismatch);
    }

    if validate_syntax::is_accessed_index_invalid(var_type.as_str(), index){
        return Err(MommyLangError::AccessViolation);
    }

    Ok(format!("{}[{}] = {};", name, index, value))
}

fn replace_array_read(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: replace [DEST] with [SRC] in [IDX]
    let dest_var  = &tokens[1];
    let src_array = &tokens[3];
    let index     = &tokens[5];

    let array_type = symbols.get(src_array).ok_or(MommyLangError::UndeclaredVariable)?;

    if validate_syntax::is_type_mismatch(array_type.as_str()){
        return Err(MommyLangError::TypeMismatch);
    }

    if validate_syntax::is_accessed_index_invalid(array_type.as_str(), index){
        return Err(MommyLangError::AccessViolation);
    }

    Ok(format!("{} = {}[{}];", dest_var, src_array, index))
}

fn replace_scalar_value(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: replace <NAME> with <VALUE>

    let name  = &tokens[constants::IDX_ASSIGN_NAME];
    let value = &tokens[constants::IDX_ASSIGN_VALUE];

    if validate_syntax::is_keyword_with_missing(&tokens){
        return Err(MommyLangError::SyntaxError);
    }

    validate_syntax::ensure_var_exists(name, symbols)?;

    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;
    let last_token = tokens.last().ok_or(MommyLangError::SyntaxError)?;

    if validate_syntax::is_replace_pointer(last_token.as_str()){
        validate_syntax::ensure_var_exists(value, symbols)?;
        return Ok(format!("{} = &{};", name, value));
    }


    if validate_syntax::is_deref_assignment(last_token.as_str()){
        if validate_syntax::is_type_pointer(var_type.as_str()){
             return Ok(format!(
                "if ({0} == NULL) {{ printf(\"Mommy Error: NULL Pointer access on '{0}'\\n\"); return 1; }} *{0} = {1};",
                name, value
            ));
        }
         return Ok(format!("{} = *{};", name, value));
    }

    Ok(format!("{} = {};", name, value))
}

fn get_c_type(raw_type: &str) -> &str {
    match raw_type {
        constants::TYPE_STRING => constants::C_TYPE_CHAR_PTR,

        // FIX: Match BOTH the user keyword ("box") and internal name ("pointer")
        t if t == constants::KW_BOX || t == constants::KW_POINTER => constants::C_TYPE_INT_PTR,

        _ => raw_type,
    }
}


pub fn insert_symbol(raw_type: &str, name: &str, symbols: &mut HashMap<String, String>) {
    if raw_type == constants::KW_BOX {
        symbols.insert(name.to_string(), constants::KW_POINTER.to_string());
    } else {
        symbols.insert(name.to_string(), raw_type.to_string());
    }
}

pub fn c_null(value: &str) -> String{
    if value == constants::KW_NULL {
        return constants::C_NULL.to_string();
    }
    value.to_string()
}