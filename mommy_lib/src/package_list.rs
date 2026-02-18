//!
//!
//! This is the package-list crate of  mommylang.
//!
//! Language Syntax: "ask if var1 comparison_operator var2"
//!
//! If you want to add a new package:
//! 1. Add a new variant to the `CStandardPackages` enum, or the `MommyCustomPackages` 
//!    enum if it is a custom package that only exists in mommylang.
//! 2. Add a new variant to the `package.rs` file, in the `add_package` function.
//! 
//!
use std::fmt;
pub enum CStandardPackages{
    InputOutput, // #include <stdio.h>
    Utilities, //#include <stdlib.h>
    UnknownPackage, // unknown

}


pub enum MommyCustomPackages{
    // tumbleweed
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