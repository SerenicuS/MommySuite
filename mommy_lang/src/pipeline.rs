use std::process::Command;

use mommy_lib::constants;
use mommy_lib::responses;

use crate::config::Config;

pub fn run_mommy_file(config: &Config) -> Result<(), String> {
    let output = if config.exe_path.contains(constants::SYM_SLASH)
        || config.exe_path.contains(constants::SYM_BACKSLASH)
    {
        config.exe_path.clone()
    } else {
        format!("{} {}", constants::PATH_DEFAULT, config.exe_path)
    };

    let status = Command::new(output)
        .status()
        .map_err(|_| responses::MommyLangError::ExecutableFile.to_string())?;

    if !status.success() {
        return Err(format!(
            "{} {}",
            responses::MommyLangError::RunFile,
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

pub fn compile_to_gcc(config: &Config) -> Result<(), String> {
    let output = Command::new(constants::CMD_GCC)
        .arg(&config.c_path)
        .arg(constants::CMD_GCC_FLAG)
        .arg(&config.exe_path)
        .output()
        .map_err(|_| responses::MommyLangError::GCCNotFound.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(error_msg)
    }
}

