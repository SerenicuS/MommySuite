
pub enum CommandType {
    Declaration,    // mayihave
    Assignment,     // replace
    Math,           // add, divide, etc.
    IO,             // say
    LoopStart,      // punishme
    LoopEnd,        // done
    LoopBreak,      // satisfied
    Condition,      // ask
    ConditionElse,  // or
    ProgramEnd,     // leave
    Unknown,        // ???
}

impl CommandType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "mayihave" => CommandType::Declaration,
            "replace" => CommandType::Assignment,
            "add" | "divide" | "subtract" | "multiply" => CommandType::Math,
            "say" => CommandType::IO,
            "punishme" => CommandType::LoopStart,
            "done" => CommandType::LoopEnd,
            "satisfied" => CommandType::LoopBreak,
            "ask" => CommandType::Condition,
            "or" => CommandType::ConditionElse,
            "leave" => CommandType::ProgramEnd,
            _ => CommandType::Unknown,
        }
    }
}