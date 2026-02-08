use crate::mommy_response;

pub fn ask(tokens: &Vec<String>) -> Result<String, mommy_response::MommyLangError>{
    if tokens.len() < 3 {
        return Err(mommy_response::MommyLangError::MissingArguments);
    }

    if tokens[1] != "if" {
        return Err(mommy_response::MommyLangError::SyntaxError); // Or "Mommy expects you to ask nicely with 'if'"
    }

    let condition = &tokens[2..].join(" ");
    Ok(format!("if ({}) {{", condition))
}

pub fn or() -> Result<String, mommy_response::MommyLangError> {
    Ok(format!("}}else {{"))
}