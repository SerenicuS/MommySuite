/// This section should be more defined in its purpose as it is just delay ms and num lol
pub const DELAY_MS_20: u64 = 20;
pub const DELAY_MS_50: u64 = 50;

pub const DELAY_MS_60: u64 = 60;
pub const DELAY_MS_80: u64 = 80;

pub const DELAY_MS_100: u64 = 100;
pub const DELAY_MS_150: u64 = 150;
pub const DELAY_MS_400: u64 = 400;
pub const DELAY_MS_500: u64 = 500;
pub const DELAY_MS_600: u64 = 600;
pub const DELAY_MS_800: u64 = 800;
pub const DELAY_MS_1000: u64 = 1000;

pub const DELAY_MS_1200: u64 = 1200;
pub const DELAY_MS_1500: u64 = 1500;


pub const JITTER_100: u64 = 100;

pub const JITTER_150: u64 = 150;

pub const TERMINAL_CLEAR: &str = "\x1B[2J\x1B[1;1H";


pub const MILLI_SLEEP_3000: u64 = 3000;

pub const REQ_CORE_FILES_NUM: usize = 4; // Should be the exact number of core files required for the suite to run. If this number is incorrect, the boot sequence will panic.

pub const SHELL_EXE_PATH: &str = if cfg!(debug_assertions) {
    "target/debug/mommy_shell.exe"
} else {
    "./mommy_shell.exe"
};

pub const OS_KEY_PATH: &str = "MOMMY_ROOT_DIR";

pub const OS_CONFIG_PROPERTIES_DIR: &str = "mommy_properties"; // I think, this should be an enum
pub const OS_CONFIG_MEMORY: &str = "mommy_conf.memory";

pub const OS_CONFIG_MEMORY_CONTENT: &str = "output=\nuser=\n";