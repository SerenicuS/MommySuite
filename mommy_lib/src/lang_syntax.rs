
pub enum MommyLangSyntax {
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
    Array,          // group
    Unknown,        // ???
}

impl MommyLangSyntax {
    pub fn from_str(token: &str) -> Self {
        match token.trim() {
            "mayihave" => MommyLangSyntax::Declaration,
            "replace" => MommyLangSyntax::Assignment,
            "add" | "divide" | "subtract" | "multiply" => MommyLangSyntax::Math,
            "say" => MommyLangSyntax::IO,
            "punishme" => MommyLangSyntax::LoopStart,
            "done" => MommyLangSyntax::LoopEnd,
            "satisfied" => MommyLangSyntax::LoopBreak,
            "ask" => MommyLangSyntax::Condition,
            "or" => MommyLangSyntax::ConditionElse,
            "leave" => MommyLangSyntax::ProgramEnd,
            "group" => MommyLangSyntax::Array, // => array
            _ => MommyLangSyntax::Unknown,
        }
    }
}

