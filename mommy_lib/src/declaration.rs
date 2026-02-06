use std::collections::HashMap;
use crate::mommy_response::MommyLangError;

pub fn create_variable(
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    // Syntax: mayihave <VAR> in <VALUE/box> as int

    if tokens.len() < 6 {
        return Err(MommyLangError::MissingArguments);
    }


    if &tokens[4] != "as"{
        return Err(MommyLangError::SyntaxError)
    }


    let in_index = tokens.iter().position(|r| r == "in")
        .ok_or(MommyLangError::SyntaxError)?;


    let raw_type = tokens.last().unwrap();

    let c_type = match raw_type.as_str(){
        "String" => "char*",
        "box" => "int*",
        _ => raw_type,
    };

    let name_index = tokens.len() - 3;
    let name = &tokens[name_index];

    if name == "int" || name == "return" || name == "void" {
        return Err(MommyLangError::InvalidVariableName);
    }

    if symbols.contains_key(name) {
        return Err(MommyLangError::VariableAlreadyExists); // Or make a new error: VariableAlreadyExists
    }

    if raw_type == "box"{
        symbols.insert(name.to_string(), "pointer".to_string());
    }
    else{
        symbols.insert(name.to_string(), raw_type.to_string());
    }

    let value_tokens = &tokens[1..in_index];
    let mut value = value_tokens.join(" ");

    if value == "null"{
        value = "NULL".to_string();
    }

    Ok(format!("{} {} = {};", c_type, name, value))
}

pub fn replace_variable (
    tokens: &Vec<String>,
    symbols: &mut HashMap<String, String>
) -> Result<String, MommyLangError> {

    if tokens.len() < 4{
    return Err(MommyLangError::MissingArguments);
    }

    let var_name = &tokens[1];
    let keyword_with = &tokens[2];
    let value = &tokens[3];

    if keyword_with != "with" {
        return Err(MommyLangError::SyntaxError)
    }

    if !symbols.contains_key(var_name) {
        return Err(MommyLangError::UndeclaredVariable)
    }


    let var_type = symbols.get(var_name).unwrap();

    if tokens.len() == 5 && tokens[4] == "address" {
        if !symbols.contains_key(value) {
            return Err(MommyLangError::UndeclaredVariable);
        }
        return Ok(format!("{} = &{};", var_name, value));
    }

    if tokens.len() == 5 && tokens[4] == "inside" {
       if var_type != "pointer"{
           return Err(MommyLangError::TypeMismatch)
       }
        return Ok(format!(
            "if ({0} == NULL) {{ \
                printf(\"Mommy Error: You tried to put value inside '{0}', but it is NULL!\\n\"); \
                return 1; \
             }} \
             *{0} = {1};",
            var_name, value
        )); // Anti segment fault
    }

    Ok(format!("{} = {};", var_name, value))


}

