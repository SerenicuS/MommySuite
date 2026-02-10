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

    if tokens.len() < constants::MIN_CREATE_VAR_ARGS {
        return Err(MommyLangError::MissingArguments);
    }

    // 1. Locate the "in" keyword to split Value vs Name
    let in_index = tokens.iter().position(|r| r == "in")
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

    if raw_type == "box" {
        symbols.insert(name.to_string(), "pointer".to_string());
    } else {
        symbols.insert(name.to_string(), raw_type.to_string());
    }

    // 5. Parse Value (Everything before "in")
    let value_tokens = &tokens[1..in_index];
    let mut value = value_tokens.join(" ");
    if value == "null" { value = "NULL".to_string(); }

    Ok(format!("{} {} = {};", c_type, name, value))
}

pub fn create_array(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: group <SIZE> in <NAME> as <TYPE>

    if tokens.len() < 6 { return Err(MommyLangError::MissingArguments); }

    // 1. Extract Parts
    let size_str = &tokens[1];
    let name = &tokens[3];
    let raw_type = &tokens[5];

    // 2. Validations
    if tokens[2] != "in" || tokens[4] != "as" { return Err(MommyLangError::SyntaxError); }
    ensure_valid_name(name)?;
    ensure_var_new(name, symbols)?;

    if size_str.parse::<usize>().is_err() {
        return Err(MommyLangError::SyntaxError); // Invalid Size
    }

    // 3. Store Metadata "array:type:size"
    let c_type = get_c_type(raw_type);
    let meta = format!("array:{}:{}", raw_type, size_str);
    symbols.insert(name.to_string(), meta);

    Ok(format!("{} {}[{}] = {{0}};", c_type, name, size_str))
}

pub fn replace(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    // Check minimal arguments for "replace x with y"
    if tokens.len() < constants::MIN_REPLACE_VAR_ARGS {
        return Err(MommyLangError::MissingArguments);
    }

    // Check if this is an Array Operation (Has "in")
    // Pattern: replace A with B in C
    if tokens.len() >= 6 && tokens[4] == "in" {

        let first_var = &tokens[1];

        // Safety: Variable must exist to check its type
        if !symbols.contains_key(first_var) {
            return Err(MommyLangError::UndeclaredVariable);
        }

        let first_type = symbols.get(first_var).unwrap();

        // --- THE SMART ROUTER ---
        if first_type.starts_with("array") {
            // Case A: The First Word is an Array -> WRITE
            // "replace ARR with VAL in IDX" -> arr[idx] = val;
            return replace_array_write(tokens, symbols);
        } else {
            // Case B: The First Word is a Variable -> READ
            // "replace VAR with ARR in IDX" -> var = arr[idx];
            return replace_array_read(tokens, symbols);
        }
    }

    // Otherwise, it is a normal Variable/Pointer assignment
    return replace_scalar_value(tokens, symbols);
}

// ================================================================
// SPECIALIZED WORKERS
// ================================================================

fn replace_array_write(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    let name = &tokens[1];   // arr
    let value = &tokens[3];  // 5
    let index = &tokens[5];  // 0

    // 1. We already know 'name' exists and is an array (checked in Router)
    let var_type = symbols.get(name).unwrap();

    // 2. Compile-Time Bounds Check
    let parts: Vec<&str> = var_type.split(':').collect();
    if let Ok(max_size) = parts[2].parse::<usize>() {
        if let Ok(idx_num) = index.parse::<usize>() {
            if idx_num >= max_size {
                return Err(MommyLangError::AccessViolation);
            }
        }
    }

    // 3. Generate C Code
    // arr[0] = 5;
    Ok(format!("{}[{}] = {};", name, index, value))
}

fn replace_array_read(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    let dest_var = &tokens[1];   // x
    let src_array = &tokens[3];  // arr
    let index = &tokens[5];      // 0

    // 1. Validate Source Array
    if !symbols.contains_key(src_array) {
        return Err(MommyLangError::UndeclaredVariable);
    }

    let array_type = symbols.get(src_array).unwrap();
    if !array_type.starts_with("array") {
        // You tried to read from something that isn't an array!
        return Err(MommyLangError::TypeMismatch);
    }

    // 2. Compile-Time Bounds Check
    let parts: Vec<&str> = array_type.split(':').collect();
    if let Ok(max_size) = parts[2].parse::<usize>() {
        if let Ok(idx_num) = index.parse::<usize>() {
            if idx_num >= max_size {
                return Err(MommyLangError::AccessViolation);
            }
        }
    }

    // 3. Generate C Code
    // x = arr[0];
    Ok(format!("{} = {}[{}];", dest_var, src_array, index))
}
fn replace_scalar_value(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {
    // Syntax: replace <NAME> with <VALUE> ...

    let name = &tokens[1];
    let value = &tokens[3];

    // 1. Basic Validation
    if tokens[2] != "with" { return Err(MommyLangError::SyntaxError); }
    ensure_var_exists(name, symbols)?;

    let var_type = symbols.get(name).unwrap();
    let last_token = tokens.last().unwrap();

    // 2. Pointer Logic
    if last_token == "address" {
        // replace p with x address
        ensure_var_exists(value, symbols)?;
        return Ok(format!("{} = &{};", name, value));
    }
    else if last_token == "inside" {
        // replace p with 10 inside
        if var_type != "pointer" { return Err(MommyLangError::TypeMismatch); }

        // Safety Check
        return Ok(format!(
            "if ({0} == NULL) {{ printf(\"Mommy Error: NULL Pointer access on '{0}'\\n\"); return 1; }} *{0} = {1};",
            name, value
        ));
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
        "String" => "char*",
        "box" => "int*",
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
        "int" | "return" | "void" | "char" | "if" | "while" => Err(MommyLangError::InvalidVariableName),
        _ => Ok(())
    }
}