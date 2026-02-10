use crate::responses;
use crate::constants;

pub fn ask(tokens: &Vec<String>) -> Result<String, responses::MommyLangError>{
    if tokens.len() < constants::MIN_ARGS_CONDITIONS_LEN{
        return Err(responses::MommyLangError::MissingArguments);
    }

    if tokens[1] != constants::CONDITIONS_IF_KEYWORD {
        return Err(responses::MommyLangError::SyntaxError); // Or "Mommy expects you to ask nicely with 'if'"
    }

    let condition = &tokens[2..].join(" ");
    Ok(format!("if ({}) {{", condition)) 
}

pub fn or() -> Result<String, responses::MommyLangError> {
    Ok(format!("{}", constants::CONDITIONS_OR_KEYWORD))
}