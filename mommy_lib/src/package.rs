use crate::package_list;
use crate::responses::MommyLangError;

pub fn add_package(package: &str) -> Result<String, MommyLangError> {

    match package.trim() {
        "listen_and_read" => Ok(format!("{}", package_list::CStandardPackages::InputOutput.to_string())),
        "ask_more" => Ok(format!("{}", package_list::CStandardPackages::Utilities.to_string())),
        _ => Err(MommyLangError::UnknownPackage),
    }
}