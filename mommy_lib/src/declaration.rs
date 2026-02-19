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

// ================================================================
// PUBLIC FUNCTIONS (The Logic)
// ================================================================

pub fn create_variable(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    if  is_args_missing_decl(tokens.len()){
        return Err(MommyLangError::MissingArguments);
    }

    let in_index = tokens.iter().position(|r| r == constants::KW_IN) // in
        .ok_or(MommyLangError::SyntaxError)?;

    let name_index = in_index + 1;
    let type_index = name_index + 2;

    if is_name_and_index_greater_than_len(name_index, type_index, tokens.len()){
        return Err(MommyLangError::SyntaxError);
    }

    let name = &tokens[name_index];
    let raw_type = &tokens[type_index];

    ensure_valid_name(name)?;
    ensure_var_new(name, symbols)?;

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

    if is_args_missing_decl(tokens.len()){
        return Err(MommyLangError::MissingArguments);
    }

    let size_str = &tokens[constants::IDX_DECL_VALUE];
    let name     = &tokens[constants::IDX_DECL_NAME];
    let raw_type = &tokens[constants::IDX_DECL_TYPE];

    ensure_valid_name(name)?;
    ensure_var_new(name, symbols)?;

    if is_invalid_array_tokens(&tokens){
        return Err(MommyLangError::SyntaxError);
    }

    if is_invalid_array_size(size_str.as_str()){
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

    if is_args_missing_decl(tokens.len()){
        return Err(MommyLangError::MissingArguments);
    }

    let size_val = &tokens[constants::IDX_DECL_VALUE];
    let name = &tokens[constants::IDX_DECL_NAME];
    let raw_type = &tokens[constants::IDX_DECL_TYPE];

    if is_invalid_array_tokens(&tokens){
        return Err(MommyLangError::SyntaxError);
    }

    ensure_valid_name(name)?;
    ensure_var_new(name, symbols)?;

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

    if is_invalid_dealloc_tokens(&tokens){
        return Err(MommyLangError::MissingArguments);
    }

    let name = &tokens[constants::IDX_DECL_VALUE];

    ensure_var_exists(name, symbols)?;

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

    ensure_var_exists(name, symbols)?;
    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;

    if is_type_mismatch(var_type.as_str()){
        return Err(MommyLangError::TypeMismatch);
    }

    if is_accessed_index_invalid(var_type.as_str(), index){
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

    if is_type_mismatch(array_type.as_str()){
        return Err(MommyLangError::TypeMismatch);
    }

    if is_accessed_index_invalid(array_type.as_str(), index){
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

    if is_keyword_with_missing(&tokens){
        return Err(MommyLangError::SyntaxError);
    }

    ensure_var_exists(name, symbols)?;

    let var_type = symbols.get(name).ok_or(MommyLangError::UndeclaredVariable)?;
    let last_token = tokens.last().ok_or(MommyLangError::SyntaxError)?;

    if is_replace_pointer(last_token.as_str()){
        ensure_var_exists(value, symbols)?;
        return Ok(format!("{} = &{};", name, value));
    }


    if is_deref_assignment(last_token.as_str()){
        if is_type_pointer(var_type.as_str()){
             return Ok(format!(
                "if ({0} == NULL) {{ printf(\"Mommy Error: NULL Pointer access on '{0}'\\n\"); return 1; }} *{0} = {1};",
                name, value
            ));
        }
         return Ok(format!("{} = *{};", name, value));
    }

    Ok(format!("{} = {};", name, value))
}


// ================================================================
// PRIVATE HELPERS (The Cleanup Crew)
// ================================================================

fn get_c_type(raw_type: &str) -> &str {
    match raw_type {
        constants::TYPE_STRING => constants::C_TYPE_CHAR_PTR,

        // FIX: Match BOTH the user keyword ("box") and internal name ("pointer")
        t if t == constants::KW_BOX || t == constants::KW_POINTER => constants::C_TYPE_INT_PTR,

        _ => raw_type,
    }
}

fn ensure_var_new(name: &str, symbols: &HashMap<String, String>) -> Result<(), MommyLangError> {
    if symbols.contains_key(name) {
        Err(MommyLangError::VariableAlreadyExists)
    } else {
        Ok(())
    }
}

fn ensure_var_exists(name: &str, symbols: &HashMap<String, String>) -> Result<(), MommyLangError> {
    if !symbols.contains_key(name) {
        Err(MommyLangError::UndeclaredVariable)
    } else {
        Ok(())
    }
}


fn ensure_valid_name(name: &str) -> Result<(), MommyLangError> {
    match name {
        constants::TYPE_INT |
        constants::C_KW_RETURN |
        constants::C_KW_VOID |
        constants::C_KW_CHAR |
        constants::KW_IF |
        constants::C_KW_WHILE => Err(MommyLangError::InvalidVariableName),

        _ => Ok(())
    }
}



/// Validate Syntax and provide C null representation
fn c_null(value: &str) -> String{
    if value == constants::KW_NULL {
        return constants::C_NULL.to_string();
    }
    value.to_string()
}

fn insert_symbol(raw_type: &str, name: &str, symbols: &mut HashMap<String, String>) {
    if raw_type == constants::KW_BOX {
        symbols.insert(name.to_string(), constants::KW_POINTER.to_string());
    } else {
        symbols.insert(name.to_string(), raw_type.to_string());
    }
}

fn is_name_and_index_greater_than_len(name_idx: usize, type_idx: usize, len: usize) -> bool{
    if name_idx >= len || type_idx >= len {
        return true
    }
    false
}

fn is_args_missing_decl(args_len: usize) -> bool{
    if args_len < constants::ARGS_MIN_DECL {
        return true
    }
    false
}


fn is_invalid_array_tokens(tokens: &Vec<String>) -> bool{
     if tokens[constants::IDX_DECL_KEY_IN] != constants::KW_IN ||
        tokens[constants::IDX_DECL_KEY_AS] != constants::KW_AS {
        return true;
    }
    false
}

fn is_invalid_array_size(size_str: &str) -> bool {
    match size_str.parse::<usize>() {
        Ok(size) => size > constants::MAX_ARRAY_SIZE,
        Err(_) => true
    }
}


fn is_type_mismatch(var_type: &str) -> bool{
     if !var_type.starts_with(constants::KW_ARRAY) &&
        !var_type.starts_with(constants::KW_HEAP) &&
        var_type != constants::TYPE_STRING &&
        var_type != constants::KW_POINTER {
        return true
    }
    false
}

fn is_accessed_index_invalid(var_type: &str, index: &str) -> bool{
    // Check both stack arrays ("array:type:size") and heap allocations ("heap:type:size")
    if var_type.starts_with(constants::KW_ARRAY) || var_type.starts_with(constants::KW_HEAP) {
        let parts: Vec<&str> = var_type.split(constants::SYM_SPLITTER).collect();
        if let Ok(max_size) = parts[2].parse::<usize>() {
            if let Ok(idx_num) = index.parse::<usize>() {
                if idx_num >= max_size {
                   return true
                }
            }
        }
    }
    false
}

fn is_keyword_with_missing(tokens: &Vec<String>) -> bool{
     if tokens[constants::IDX_ASSIGN_KEY_WITH] != constants::KW_WITH {
        return true
    }
    false
}


fn is_replace_pointer(last_token: &str) -> bool{
    if last_token == constants::KW_ADDR{
        return true
    }
    false
}

fn is_type_pointer(var_type: &str) -> bool{
    if var_type == constants::KW_POINTER {
        return true
    }
    false
}

fn is_deref_assignment(last_token: &str) -> bool{
    if last_token == constants::KW_DEREF{
        return true
    }
    false
}

fn is_invalid_dealloc_tokens(tokens: &Vec<String>) -> bool{
    if tokens.len() < constants::ARGS_MIN_LEN{
        return true
    }
    false
}
