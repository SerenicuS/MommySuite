pub fn ask(tokens: &Vec<String>) -> String{
    let condition = &tokens[2..].join(" ");
    format!("if ({}) {{", condition)
}

pub fn or() -> String{
    format!("else {{")
}