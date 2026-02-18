//!
//!
//! This is the math crate of mommylang.
//!
//! Language Syntax: "target = target operator value/variable"
//!
//! Note:
//! 1. It can only do operations on one variable at a time, such as "x = x + 10",
//!    which is sort of similar to assembly language syntax.
//! 2. Pemdas or AST is not supported. If you want to do multiple operations,
//!    you need to chain them sequentially.
//! 3. There is no support for variable type conversion.
//!
//!
//!
//!
//!
//!
//!
//!
use std::collections::HashMap;
use crate::responses::{MommyLangError};
use crate::constants;

pub fn calculate_two(target: &str, operator: &str, value: &str, symbols: &HashMap<String, String>) -> Result<String, MommyLangError> {
    // Let us validate it first, because it might be an illegal operation like walter's meth lab.
    validate_operation(target, operator, value, symbols)?;
    Ok(format!("{} = {} {} {};", target, target, operator, value))
}


fn validate_operation(
    target: &str,
    operator: &str,
    value: &str,
    symbols: &HashMap<String, String>
) -> Result<(), MommyLangError> {

    let var_type = match symbols.get(target) { // Does the target variable exist?
        Some(t) => t,
        None => return Err(MommyLangError::UndeclaredVariable),
    };

    if !symbols.contains_key(value){ // It might be a variable, check our hashmap.....
        if value.parse::<f64>().is_err(){ // if it is not a variable, can it be a number?
            return Err(MommyLangError::UndeclaredVariable);
        }
    }

    if operator == constants::C_OP_DIV && value == constants::C_VAL_ZERO { // No division on 0!
        return Err(MommyLangError::DivideByZero);
    }

    if var_type == constants::TYPE_STRING  || var_type == constants::C_TYPE_CHAR_PTR { // No math on Strings!
    return Err(MommyLangError::MathOnString);
    }

    Ok(()) // Valid operation!
}
