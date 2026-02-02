use std::fmt;

pub enum MommyErrorResponse{
    // General
    MissingArguments,
    InvalidVariableName,
    UndeclaredVariable,
    TypeMismatch,
    SyntaxError,
    UnclosedBlock,
    

    //ALU
    MathOnString,
    DivideByZero,
}
impl fmt::Display for MommyErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::MissingArguments => write!(f, "{}", "I told you to properly finish what you want to say."),
            Self::UndeclaredVariable => write!(f, "{}", "I told you to name your things properly."),
            Self::InvalidVariableName => write!{f, "{}", "Really? Trying to name things that I prohibit you to use?"},
            Self::TypeMismatch => write!(f, "{}", "Matching types should be easy for you, yet you act as if it is a calculus"),
            Self::SyntaxError => write!(f, "{}", "You deal with this problem, I taught you enough like an adult"),
            Self::UnclosedBlock => write!(f, "{}", "Do you know how to use punctuations?"),

            Self::MathOnString => write!(f, "You cannot do math on words. This isn't Algebra class."),
            Self::DivideByZero => write!(f, "Zero? You want to divide by ZERO? Get out."),

        }
    }
}