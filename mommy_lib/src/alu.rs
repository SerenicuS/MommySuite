use std::collections::HashMap;
// Arithmetic logic unit
use crate::mommy_response::{MommyLangError};

pub fn calculate_two(target: &str, operator: &str, value: &str, symbols: &HashMap<String, String>) -> Result<String, MommyLangError> {
    // calculate("x", "*", "10") -> "x = x * 10;"
    validate_operation(target, operator, value, symbols)?;
    Ok(format!("{} = {} {} {};", target, target, operator, value))
}


fn validate_operation(
    target: &str,
    operator: &str,
    value: &str,
    symbols: &HashMap<String, String>
) -> Result<(), MommyLangError> {


    // Variable does not exist
    let var_type = match symbols.get(target) {
        Some(t) => t,
        None => return Err(MommyLangError::UndeclaredVariable),
    };

    if let Some(first_char) = value.chars().next() { // for negative
        let is_negative_number = first_char == '-' && value.len() > 1 && value.chars().nth(1).unwrap().is_digit(10);

        if !first_char.is_digit(10) && !is_negative_number {
            if !symbols.contains_key(value) {
                return Err(MommyLangError::UndeclaredVariable);
            }
        }
    }


    // Trying to do math in strings
    if var_type == "String" || var_type == "char*" {
        return Err(MommyLangError::MathOnString);
    }


    // Dividing 0
    if operator == "/" && value == "0" {
        return Err(MommyLangError::DivideByZero);
    }


    // I do not know what this is
    if let Some(first_char) = value.chars().next() {
        if !first_char.is_digit(10) {
            if !symbols.contains_key(value) {
                return Err(MommyLangError::UndeclaredVariable);
            }
        }
    }

    Ok(())
}