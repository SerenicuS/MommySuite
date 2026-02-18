//!
//!
//! This is the condition crate of  mommylang.
//!
//! Language Syntax: "ask if var1 comparison_operator var2"
//!
//! Note:
//! 1. It can only do one condition at a time, such as "ask if grade > 90"
//!
//! 2. Logical operators (&&, ||) and parentheses are not supported.
//!
//! 3. Boolean is not supported as a type, but you can use 0 and 1 to represent false and true lol.
//!
//!
//!
//!
//!
//!
//!
//!
use crate::responses;
use crate::constants;

pub fn ask(tokens: &Vec<String>) -> Result<String, responses::MommyLangError>{

    if tokens.len() < constants::ARGS_MIN_COND {
        return Err(responses::MommyLangError::MissingArguments);
    }

    if tokens[constants::IDX_COND_IF] != constants::KW_IF  || tokens[constants::INDX_COND_ASK] != constants::KW_ASK {
        return Err(responses::MommyLangError::SyntaxError);
    }
    let condition = tokens[2..].join(constants::SYM_WHITESPACE);
    
    Ok(format!("if ({}) {{", condition))
}

pub fn or() -> Result<String, responses::MommyLangError> {
    Ok(constants::KW_ELSE_BLOCK.to_string())
}