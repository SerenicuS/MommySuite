// Arithmetic logic unit
use std::collections::HashMap;
use crate::responses::{MommyLangError};
use crate::constants;
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


    // Does the target variable exist?
    let var_type = match symbols.get(target) {
        Some(t) => t,
        None => return Err(MommyLangError::UndeclaredVariable),
    };


    if !symbols.contains_key(value){ // Is it a variable?
        if value.parse::<f64>().is_err(){ // if it is not a variable, can it be a number?
            return Err(MommyLangError::UndeclaredVariable);
        }
    }

    // Trying to do math in strings
    if var_type == constants::TYPE_STRING  || var_type == constants::C_TYPE_CHAR_PTR {
        return Err(MommyLangError::MathOnString);
    }


    // Dividing 0
    if operator == constants::C_OP_DIV && value == constants::C_VAL_ZERO {
        return Err(MommyLangError::DivideByZero);
    }

    Ok(())
}
