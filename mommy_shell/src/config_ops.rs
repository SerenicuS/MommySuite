use std::env;

use mommy_lib::config;
use mommy_lib::responses;
use mommy_lib::shell_format::print_line;


pub fn shell_change_code_dir(new_dir: &str, mommy_settings: &mut config::MommySettings) {
    let current_working_dir = env::current_dir().unwrap();
    let absolute_target = current_working_dir.join(new_dir);
    mommy_settings.output_directory = absolute_target.to_string_lossy().to_string();

    if let Err(_) = mommy_settings.save_path() {
        print_line(responses::MommyShellError::ConfigSaveError);
    } else {
        print_line(responses::MommyShellOk::ConfigUpdated);
        print_line(format!("(Mommy set the output to: {})", mommy_settings.output_directory));
    }
}

pub fn shell_change_username(user_name: &str, mommy_settings: &mut config::MommySettings) {
    match mommy_settings.save_user(user_name.trim()) {
        Ok(_) => {
            mommy_settings.user_name = user_name.trim().to_string();
            print_line(format!("{} {}", responses::MommyUI::AcceptNewName, user_name));
        }
        Err(_) =>  {
            print_line(responses::MommyUI::BrainError)
        },

    }
}

