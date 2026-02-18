//!
//!
//! This is the header crate of  mommylang.
//!
//! Language Syntax: "makeme package_name"
//!
//! Note:
//! 1. Limited packages support for now, some packages might be custom-made for mommylang only.
//!
//!


use crate::package_list;
use crate::responses::MommyLangError;

pub fn add_package(package: &str) -> Result<String, MommyLangError> {

    match package.trim() {
        "listen_and_read" => Ok(format!("{}", package_list::CStandardPackages::InputOutput.to_string())),
        "ask_more" => Ok(format!("{}", package_list::CStandardPackages::Utilities.to_string())),
        _ => Err(MommyLangError::UnknownPackage),
    }
}