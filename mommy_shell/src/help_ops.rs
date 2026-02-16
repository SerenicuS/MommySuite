use mommy_lib::constants;
use mommy_lib::shell_format::print_line;

pub fn shell_print_basic_help() {
    print_line(constants::SHELL_BASIC_COMMANDS);
}

pub fn shell_print_advance_help() {
    print_line(constants::SHELL_ADVANCE_COMMANDS);
}

