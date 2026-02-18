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
    // Syntax: mayihave <VALUE> in <NAME> as <TYPE>

    if tokens.len() < constants::ARGS_MIN_DECL {
        return Err(MommyLangError::MissingArguments);
    }

    // 1. Locate the "in" keyword to split Value vs Name
    let in_index = tokens.iter().position(|r| r == constants::KW_IN) // in
        .ok_or(MommyLangError::SyntaxError)?;

    // 2. Extract Parts
    let name_index = in_index + 1; // Name is right after "in"
    let type_index = name_index + 2; // Type is after "as"

    // Safety check indices
    if name_index >= tokens.len() || type_index >= tokens.len() {
        return Err(MommyLangError::SyntaxError);
    }

    let name = &tokens[name_index];
    let raw_type = &tokens[type_index];

    // 3. Validations (Using Helpers)
    ensure_valid_name(name)?;
    ensure_var_new(name, symbols)?;

    // 4. Convert Type & Store
    let c_type = get_c_type(raw_type);

    if raw_type == constants::KW_BOX {
        symbols.insert(name.to_string(), constants::KW_POINTER.to_string());
    } else {
        symbols.insert(name.to_string(), raw_type.to_string());
    }

    // 5. Parse Value (Everything before "in")
    let value_tokens = &tokens[1..in_index];
    let mut value = value_tokens.join(constants::SYM_WHITESPACE);

    // Fix: Use the new C_NULL constant
    if value == constants::KW_NULL {
        value = constants::C_NULL.to_string();
    }

    Ok(format!("{} {} = {};", c_type, name, value))
}

pub fn create_array(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: group <SIZE> in <NAME> as <TYPE>

    if tokens.len() < constants::ARGS_MIN_DECL {
        return Err(MommyLangError::MissingArguments);
    }

    // 1. Extract Parts (Using new short indices)
    let size_str = &tokens[constants::IDX_DECL_VALUE];
    let name     = &tokens[constants::IDX_DECL_NAME];
    let raw_type = &tokens[constants::IDX_DECL_TYPE];

    // 2. Validations
    if tokens[constants::IDX_DECL_KEY_IN] != constants::KW_IN ||
        tokens[constants::IDX_DECL_KEY_AS] != constants::KW_AS {
        return Err(MommyLangError::SyntaxError);
    }

    ensure_valid_name(name)?;
    ensure_var_new(name, symbols)?;

    if size_str.parse::<usize>().is_err() {
        return Err(MommyLangError::SyntaxError); // Invalid Size
    }

    // 3. Store Metadata "array:type:size"
    let meta = format!("{}:{}:{}", constants::KW_ARRAY, raw_type, size_str);
    symbols.insert(name.to_string(), meta);

    let c_type = match raw_type.as_str(){
        constants::TYPE_ASCII => constants::TYPE_INT,
        _ => get_c_type(&raw_type),
    };

    Ok(format!("{} {}[{}] = {{0}};", c_type, name, size_str))
}

pub fn allocate_heap(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: ibegyou <SIZE> in <NAME> as <TYPE>

    if tokens.len() < constants::ARGS_MIN_DECL{
        return Err(MommyLangError::MissingArguments);
    }

    let size_val = &tokens[constants::IDX_DECL_VALUE];
    let name = &tokens[constants::IDX_DECL_NAME];
    let raw_type = &tokens[constants::IDX_DECL_TYPE];

    if tokens[constants::IDX_DECL_KEY_IN] != constants::KW_IN || tokens[constants::IDX_DECL_KEY_AS] != constants::KW_AS {
        return Err(MommyLangError::SyntaxError);
    }

    ensure_valid_name(name)?;
    ensure_var_new(name, symbols)?;

    let c_type = match raw_type.as_str() {
        constants::TYPE_ASCII => constants::TYPE_INT,
        _ => get_c_type(raw_type),
    };

    if raw_type == constants::TYPE_ASCII {
        // TODO: Consider how heap-ascii should interact with replace/deref semantics.
        let meta = format!("{}:{}:{}", constants::KW_ARRAY, raw_type, size_val);
        symbols.insert(name.to_string(), meta);
    } else {
        symbols.insert(name.to_string(), constants::KW_POINTER.to_string());
    }

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
    // Example: takeitback ptr
    // C Output: free(ptr); ptr = NULL;
    if tokens.len() < constants::ARGS_MIN_LEN{
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

    // Minimum needed: "replace x with y" (4 tokens)
    if tokens.len() < constants::ARGS_MIN_ASSIGN {
        return Err(MommyLangError::MissingArguments);
    }

    // CASE A: WRITE to Array (replace arr in idx with val)
    // Check if "in" is the 3rd word (Index 2)
    if tokens[constants::IDX_ARR_KEY_IN] == constants::KW_IN {
        return replace_array_write(tokens, symbols);
    }

    // CASE B: READ from Array (replace val with arr in idx)
    // Check if "in" is the 5th word (Index 4) AND 3rd word is "with"
    // Note: We reuse IDX_ARR_KEY_WITH (4) for the 'in' position in this specific syntax
    if tokens.len() >= constants::ARGS_MIN_ARR_ASSIGN
        && tokens[constants::IDX_ASSIGN_KEY_WITH] == constants::KW_WITH
        && tokens[constants::IDX_ARR_KEY_WITH] == constants::KW_IN {
        return replace_array_read(tokens, symbols);
    }

    // CASE C: Normal Variable (replace x with y)
    // Check if "with" is the 3rd word (Index 2)
    if tokens[constants::IDX_ASSIGN_KEY_WITH] == constants::KW_WITH {
        return replace_scalar_value(tokens, symbols);
    }

    return Err(MommyLangError::SyntaxError);
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

    let name  = &tokens[1]; // Name is at 1
    let index = &tokens[constants::IDX_ARR_INDEX];
    let value = &tokens[constants::IDX_ARR_VALUE];

    ensure_var_exists(name, symbols)?;
    let var_type = symbols.get(name).unwrap();

   if !var_type.starts_with(constants::KW_ARRAY) &&
        var_type != constants::TYPE_STRING &&
        var_type != constants::KW_POINTER {
        return Err(MommyLangError::TypeMismatch);
   }

    if var_type.starts_with(constants::KW_ARRAY) {
        let parts: Vec<&str> = var_type.split(constants::SYM_SPLITTER).collect();
        if let Ok(max_size) = parts[2].parse::<usize>() {
            if let Ok(idx_num) = index.parse::<usize>() {
                if idx_num >= max_size {
                    return Err(MommyLangError::AccessViolation);
                }
            }
        }
    }

    // Output: word[0] = 'R';
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

    if !symbols.contains_key(src_array) {
        return Err(MommyLangError::UndeclaredVariable);
    }

    let array_type = symbols.get(src_array).unwrap();

  if !array_type.starts_with(constants::KW_ARRAY) &&
        array_type != constants::TYPE_STRING &&
        array_type != constants::KW_POINTER {
        return Err(MommyLangError::TypeMismatch);
    }

    if array_type.starts_with(constants::KW_ARRAY) {
        let parts: Vec<&str> = array_type.split(constants::SYM_SPLITTER).collect();
        if let Ok(max_size) = parts[2].parse::<usize>() {
            if let Ok(idx_num) = index.parse::<usize>() {
                if idx_num >= max_size {
                    return Err(MommyLangError::AccessViolation);
                }
            }
        }
    }

    Ok(format!("{} = {}[{}];", dest_var, src_array, index))
}

fn replace_scalar_value(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: replace <NAME> with <VALUE> ...

    let name  = &tokens[constants::IDX_ASSIGN_NAME];
    let value = &tokens[constants::IDX_ASSIGN_VALUE];

    // 1. Basic Validation
    if tokens[constants::IDX_ASSIGN_KEY_WITH] != constants::KW_WITH {
        return Err(MommyLangError::SyntaxError);
    }
    ensure_var_exists(name, symbols)?;

    let var_type = symbols.get(name).unwrap();
    let last_token = tokens.last().unwrap();

    // 2. Pointer Logic
    if last_token == constants::KW_ADDR {
        // replace p with x address
        ensure_var_exists(value, symbols)?;
        return Ok(format!("{} = &{};", name, value));
    }
   else if last_token == constants::KW_DEREF {
        // CASE A: Writing TO a pointer (replace ptr with 10 inside)
        // logic: *ptr = 10;
        if var_type == constants::KW_POINTER {
            // Safety Check
            return Ok(format!(
                "if ({0} == NULL) {{ printf(\"Mommy Error: NULL Pointer access on '{0}'\\n\"); return 1; }} *{0} = {1};",
                name, value
            ));
        }
        // CASE B: Reading FROM a pointer (replace x with ptr inside)
        // logic: x = *ptr;
        else {
            return Ok(format!("{} = *{};", name, value));
        }
    }

    // 3. Normal Assignment
    Ok(format!("{} = {};", name, value))
}

// ================================================================
// PRIVATE HELPERS (The Cleanup Crew)
// ================================================================

/// Converts Mommy types to C types
fn get_c_type(raw_type: &str) -> &str {
    match raw_type {
        constants::TYPE_STRING => constants::C_TYPE_CHAR_PTR,

        // FIX: Match BOTH the user keyword ("box") and internal name ("pointer")
        t if t == constants::KW_BOX || t == constants::KW_POINTER => constants::C_TYPE_INT_PTR,

        _ => raw_type,
    }
}
/// Ensures the variable is NOT in the symbol table (for Declarations)
fn ensure_var_new(name: &str, symbols: &HashMap<String, String>) -> Result<(), MommyLangError> {
    if symbols.contains_key(name) {
        Err(MommyLangError::VariableAlreadyExists)
    } else {
        Ok(())
    }
}

/// Ensures the variable IS in the symbol table (for Usage)
fn ensure_var_exists(name: &str, symbols: &HashMap<String, String>) -> Result<(), MommyLangError> {
    if !symbols.contains_key(name) {
        Err(MommyLangError::UndeclaredVariable)
    } else {
        Ok(())
    }
}

/// Blacklist checks
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