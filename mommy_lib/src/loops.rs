
pub fn for_loop(tokens: &Vec<String>) -> String {
    // For loops
    let repeat_count = &tokens[1];
    format!("for (int i = 0; i < {}; i++) {{", repeat_count)
}

pub fn done() -> String {
    "}".to_string()
}

pub fn satisfied() -> String {
    "break;".to_string()
}


pub fn while_loop(tokens: &Vec<String>) -> String{
    // Logic: "punishmeif i < 10" -> "while (i < 10) {"
    // tokens[0] is "punishmeif"
    let condition = tokens[1..].join(" ");
    format!("while ({}) {{", condition)
}
