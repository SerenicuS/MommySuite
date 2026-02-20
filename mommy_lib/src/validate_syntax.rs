use std::collections::HashMap;
use crate::constants;
use crate::responses::MommyLangError;

// ================================================================
// VARIABLE VALIDATION (Symbol Table Operations)
// ================================================================

pub fn ensure_var_new(name: &str, symbols: &HashMap<String, String>) -> Result<(), MommyLangError> {
    if symbols.contains_key(name) {
        Err(MommyLangError::VariableAlreadyExists)
    } else {
        Ok(())
    }
}

pub fn ensure_var_exists(name: &str, symbols: &HashMap<String, String>) -> Result<(), MommyLangError> {
    if !symbols.contains_key(name) {
        Err(MommyLangError::UndeclaredVariable)
    } else {
        Ok(())
    }
}

pub fn ensure_valid_name(name: &str) -> Result<(), MommyLangError> {
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

// ================================================================
// TYPE VALIDATION
// ================================================================

pub fn is_type_mismatch(is_type: &str) -> bool{
    if !is_type.starts_with(constants::KW_ARRAY) &&
       !is_type.starts_with(constants::KW_HEAP) &&
        is_type != constants::TYPE_STRING &&
        is_type != constants::KW_POINTER {
        return true
    }
    false
}

pub fn is_type_pointer(var_type: &str) -> bool{
    if var_type == constants::KW_POINTER {
        return true
    }
    false
}


// ================================================================
// ARGS VALIDATION (Arguments & Tokens)
// ================================================================


pub fn is_name_and_index_greater_than_len(name_idx: usize, type_idx: usize, len: usize) -> bool{
    if name_idx >= len || type_idx >= len {
        return true
    }
    false
}

pub fn is_missing_decl_args(args_len: usize) -> bool{
    if args_len < constants::ARGS_MIN_DECL {
        return true
    }
    false
}

pub fn is_missing_say_args(tokens_len: usize) -> bool{
    if tokens_len < constants::ARGS_MIN_IO{
        return true;
    }
    false
}

// ================================================================
// ARRAY VALIDATION
// ================================================================

pub fn is_invalid_array_tokens(tokens: &Vec<String>) -> bool{
     if tokens[constants::IDX_DECL_KEY_IN] != constants::KW_IN ||
        tokens[constants::IDX_DECL_KEY_AS] != constants::KW_AS {
        return true;
    }
    false
}

pub fn is_invalid_array_size(size_str: &str) -> bool {
    match size_str.parse::<usize>() {
        Ok(size) => size > constants::MAX_ARRAY_SIZE,
        Err(_) => true
    }
}

pub fn is_accessed_index_invalid(var_type: &str, index: &str) -> bool{
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

pub fn is_output_be_wildcard(array_type: &str, idx_num: usize, max_size: usize) -> bool {
    if (array_type.starts_with(constants::KW_ARRAY) || array_type.starts_with(constants::KW_HEAP)) &&
        idx_num >= max_size && max_size != constants::SIZE_UNKNOWN{
        return true
    }

    false
}



pub fn is_kw_all(idx: &str) -> bool{
    if idx == constants::KW_ALL{
        return true;
    }
    false
}




// ================================================================
// ASSIGNMENT VALIDATION (Replace/Pointer Operations)
// ================================================================

pub fn is_keyword_with_missing(tokens: &Vec<String>) -> bool{
     if tokens[constants::IDX_ASSIGN_KEY_WITH] != constants::KW_WITH {
        return true
    }
    false
}

pub fn is_replace_pointer(last_token: &str) -> bool{
    if last_token == constants::KW_ADDR{
        return true
    }
    false
}

pub fn is_deref_assignment(last_token: &str) -> bool{
    if last_token == constants::KW_DEREF{
        return true
    }
    false
}

// ================================================================
// MEMORY MANAGEMENT VALIDATION (Heap Deallocation)
// ================================================================

pub fn is_invalid_dealloc_tokens(tokens: &Vec<String>) -> bool{
    if tokens.len() < constants::ARGS_MIN_LEN{
        return true
    }
    false
}

// ================================================================
// SWAPPING VALUES
// ================================================================

pub fn select_inner_type(parts_len: usize, parts: &str) -> &str{
     if parts_len > 1 {
         return parts
     }
    constants::TYPE_STRING
}

pub fn select_max_size(parts_len: usize, parts: &str) -> usize {
    if parts_len > 2 {
        if let Ok(size) = parts.parse::<usize>() {
            return size
        }
    }
    0
}