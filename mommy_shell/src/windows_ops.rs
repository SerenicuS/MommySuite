use std::process::Command;

use mommy_lib::responses;
use mommy_lib::shell_format::print_line;

pub fn shell_windows_call(windows_command: &str) {
    match windows_command {
        "ipconfig" => windows_command_console_output(windows_command),
        _ => print_line(responses::MommyShellError::ExternalIPConfigCallFail),
    }
}

fn windows_command_console_output(var: &str) {
    match Command::new(var).output() {
        Ok(output) => {
            let console_output = String::from_utf8_lossy(&output.stdout);
            print_line(console_output.trim_end());
        }
        Err(_) => print_line(responses::MommyShellError::ExternalCommandFailed),
    }
}

