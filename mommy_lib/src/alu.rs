use std::collections::HashMap;
// Arithmetic logic unit
use crate::mommy_response::MommyLangErrorResponse;

pub fn calculate_two(target: &str, operator: &str, value: &str, symbols: &HashMap<String, String>) -> Result<String, MommyLangErrorResponse> {
    // calculate("x", "*", "10") -> "x = x * 10;"
    validate_operation(target, operator, value, symbols)?;
    Ok(format!("{} = {} {} {};", target, target, operator, value))
}


fn validate_operation(
    target: &str,
    operator: &str,
    value: &str,
    symbols: &HashMap<String, String>
) -> Result<(), MommyLangErrorResponse> {

    // Variable does not exist
    let var_type = match symbols.get(target) {
        Some(t) => t,
        None => return Err(MommyLangErrorResponse::UndeclaredVariable),
    };


    // Trying to do math in strings
    if var_type == "String" || var_type == "char*" {
        return Err(MommyLangErrorResponse::MathOnString);
    }


    // Dividing 0
    if operator == "/" && value == "0" {
        return Err(MommyLangErrorResponse::DivideByZero);
    }


    // I do not know what this is
    if let Some(first_char) = value.chars().next() {
        if !first_char.is_digit(10) {
            if !symbols.contains_key(value) {
                return Err(MommyLangErrorResponse::UndeclaredVariable);
            }
        }
    }

    Ok(())
}