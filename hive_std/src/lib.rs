// Insert Token


pub fn insert_token(input: &str) -> Vec<String>{
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut inside_quote = false;

    for c in input.chars(){
        match c{
            '"' =>{
                inside_quote = !inside_quote;
                current_token.push(c);
            }

            ' ' | '\t' | '\r' | '\n' if !inside_quote => {
                if !current_token.is_empty(){
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }

            _ => {
                current_token.push(c);
            }
        }
    }

    if !current_token.is_empty(){
        tokens.push(current_token);
    }

    tokens

}