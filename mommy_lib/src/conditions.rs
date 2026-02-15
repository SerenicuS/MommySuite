use crate::responses;
use crate::constants;

pub fn ask(tokens: &Vec<String>) -> Result<String, responses::MommyLangError>{
    // 1. Check if we have enough words ("ask if x")
    if tokens.len() < constants::ARGS_MIN_COND {
        return Err(responses::MommyLangError::MissingArguments);
    }

    // 2. Ensure the user said "if"
    // Note: tokens[0] is "ask", tokens[1] must be "if"
    if tokens[1] != constants::KW_IF {
        return Err(responses::MommyLangError::SyntaxError);
    }

    // 3. Construct the C condition
    // Join the rest of the tokens (x == 10) with spaces
    let condition = tokens[2..].join(constants::SYM_WHITESPACE);

    Ok(format!("if ({}) {{", condition))
}

pub fn or() -> Result<String, responses::MommyLangError> {
    // Just return "} else {"
    Ok(constants::KW_ELSE_BLOCK.to_string())
}