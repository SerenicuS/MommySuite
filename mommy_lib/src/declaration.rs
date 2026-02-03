use std::collections::HashMap;
use crate::mommy_response::MommyLangErrorResponse;

pub fn may_i_have(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangErrorResponse> {


    if tokens.len() < 6 {
        return Err(MommyLangErrorResponse::MissingArguments);
    }

    let in_index = tokens.iter().position(|r| r == "in")
        .ok_or(MommyLangErrorResponse::SyntaxError)?;

    let raw_type = tokens.last().unwrap();
    let c_type = if raw_type == "String" { "char*" } else { raw_type };

    let name_index = tokens.len() - 3;
    let name = &tokens[name_index];

    if name == "int" || name == "return" {
        return Err(MommyLangErrorResponse::InvalidVariableName);
    }

    symbols.insert(name.to_string(), raw_type.to_string()); // Save the variable name

    let value_tokens = &tokens[1..in_index];
    let value = &value_tokens.join(" ");

    Ok(format!("{} {} = {};", c_type, name, value))
}