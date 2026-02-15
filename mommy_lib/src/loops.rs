// loops.rs
use crate::constants;

pub fn for_loop(tokens: &Vec<String>) -> String {
    // Syntax: punishme <COUNT>
    // Logic: "punishme 10" -> "for (int i = 0; i < 10; i++) {"
    let repeat_count = &tokens[constants::IDX_LOOP_COUNT];
    format!("for (int i = 0; i < {}; i++) {{", repeat_count)
}

pub fn done() -> String {
    constants::C_MAIN_END.to_string() // "}"
}

pub fn satisfied() -> String {
    format!("break;")
}


pub fn while_loop(tokens: &Vec<String>) -> String{
    // Logic: "punishmeif i < 10" -> "while (i < 10) {"
    // tokens[0] is constants::KW_LOOP_IF ("punishmeif")

    let condition = tokens[1..].join(constants::SYM_WHITESPACE);
    format!("while ({}) {{", condition)
}