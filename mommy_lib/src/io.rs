use std::collections::HashMap;
use crate::mommy_response::MommyLangError;
pub fn say(tokens: &Vec<String>, symbols: &HashMap<String, String>) -> Result<String, MommyLangError> {
   if tokens.len() < 2{
       return Err(MommyLangError::MissingArguments)
   }

    let message = &tokens[1];


    // Check if it's a string literal (starts with ")
    if message.starts_with("\"") {
        let full_msg = tokens[1..].join(" ");
        Ok(format!("printf(\"%s\\n\", {});", full_msg))
    } else {
        let var_type = symbols.get(message).ok_or(MommyLangError::UndeclaredVariable)?;

        if var_type == "String" {
            Ok(format!("printf(\"%s\\n\", {});", message))
        } else if var_type == "float" {
            Ok(format!("printf(\"%f\\n\", {});", message))
        } else if var_type == "pointer"{
            Ok(format!("printf(\"%p\\n\", (void*){});", message))
        }
        else {
            Ok(format!("printf(\"%d\\n\", {});", message))
        }
    }
}

