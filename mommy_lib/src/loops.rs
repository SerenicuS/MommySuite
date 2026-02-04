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