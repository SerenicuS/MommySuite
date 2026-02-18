
pub enum MommyLangSyntax {
    Declaration,    // mayihave
    Assignment,     // replace
    Math,           // add, divide, etc.
    IO,             // say
    LoopStartBasic, // punishme
    LoopEnd,        // done
    LoopBreak,      // satisfied
    Condition,      // ask
    ConditionElse,  // or
    ProgramEnd,     // leave
    Array,          // group
    Unknown,        // ???
    LoopStartCondition, // punishmeif
    String,         // ascii
    Malloc,         //ibegyou
    FreeMalloc,     // takeitback
    ReadInput,      // listen
    IncludeLib,     // makeme
}

impl MommyLangSyntax {
    pub fn from_str(token: &str) -> Self {
        match token.trim() {
            "mayihave" => MommyLangSyntax::Declaration,
            "replace" => MommyLangSyntax::Assignment,
            "add" | "divide" | "subtract" | "multiply" | "mod" => MommyLangSyntax::Math,
            "say" => MommyLangSyntax::IO,
            "punishme" => MommyLangSyntax::LoopStartBasic,
            "done" => MommyLangSyntax::LoopEnd,
            "satisfied" => MommyLangSyntax::LoopBreak,
            "ask" => MommyLangSyntax::Condition,
            "or" => MommyLangSyntax::ConditionElse,
            "leave" => MommyLangSyntax::ProgramEnd,
            "group" => MommyLangSyntax::Array, // => array
            "punishmeif" => MommyLangSyntax::LoopStartCondition,
            "ascii" => MommyLangSyntax::String,
            "ibegyou" => MommyLangSyntax::Malloc,
            "takeitback" => MommyLangSyntax::FreeMalloc,
            "listen" => MommyLangSyntax::ReadInput,
            "makeme" => MommyLangSyntax::IncludeLib,
            _ => MommyLangSyntax::Unknown,
        }
    }
}

