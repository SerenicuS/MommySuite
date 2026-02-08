// Arithmetic logic unit
use std::collections::HashMap;
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


    // Does the variable exist?
    let var_type = match symbols.get(target) {
        Some(t) => t,
        None => return Err(MommyLangError::UndeclaredVariable),
    };


    if !symbols.contains_key(value){ // Is it a variable name?
        if value.parse::<f64>().is_err(){ // Can it be parsed like a number?(check if it is fully number)
            return Err(MommyLangError::UndeclaredVariable);
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

    Ok(())
}