#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScopeType {
    Loop,       // "punishme"
    Condition,  // "ask"
    Alternative // "or" (We track this to prevent double 'or')
}