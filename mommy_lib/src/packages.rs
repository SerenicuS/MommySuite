use std::fmt;

// Standard C libraries
pub enum CStandardPackages{
    InputOutput, // #include <stdio.h>
    Utilities, //#include <stdlib.h>
    UnknownPackage, // unknown

}


pub enum MommyCustomPackages{

}

impl fmt::Display for CStandardPackages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CStandardPackages::InputOutput => {
                write!(f, "#include <stdio.h>")
            }
            CStandardPackages::Utilities => {
                write!(f, "#include <stdlib.h>")
            }
            CStandardPackages::UnknownPackage => {
                write!(f, "")
            }
        }
    }
}