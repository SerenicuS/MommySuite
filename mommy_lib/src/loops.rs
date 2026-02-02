pub fn punish_me(tokens: &Vec<String>) -> String {
    let repeat_count = &tokens[1];
    // NOTE: You will need to fix the 'i' variable collision later,
    // but for now, this moves the logic out of main.
    format!("for (int i = 0; i < {}; i++) {{", repeat_count)
}

pub fn done() -> String {
    "}".to_string()
}