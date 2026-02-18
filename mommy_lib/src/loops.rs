//!
//!
//! This is the loop crate of  mommylang.
//!
//! Language Syntax for (for (int i = 0; i < {}; i++)): "punishme count/variable"
//! Language Syntax for (while (condition)): "punishmeif var1 operator var2"
//! 
//! Known Issues:
//! 1. The "punishme" loop does not support nested loop as it only uses 1 variable "i" as the loop counter. If you want to do nested loops, you can use "punishmeif" with a condition that depends on the outer loop variable,
//!    such as "punishmeif i < 10 && j < 5", but it might get messy.(I recommend to stick to "punishmeif" for nested loops.
//! 2. Why there are no loop checks here lol?
//! 
//! 

use crate::constants;

pub fn for_loop(tokens: &Vec<String>) -> String {
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

    let condition = tokens[1..].join(constants::SYM_WHITESPACE);
    format!("while ({}) {{", condition)
}