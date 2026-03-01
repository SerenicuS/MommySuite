mod suite_constants;
mod boot_loader_animations;
mod loader_animations;
mod filesystem_manifest;
mod os_responses;

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, thread};
use std::time::{Duration};
use std::env;
use std::sync::OnceLock;
use mommy_lib::config::MommySettings;
use mommy_lib::responses;
use crate::boot_loader_animations::{MommyBootloader};
use crate::loader_animations::{AnimationType};


static APP_CONTEXT: OnceLock<AppContext> = OnceLock::new();
#[derive(Debug)]
pub struct AppContext {
    pub root_dir: PathBuf,
}

impl AppContext {
    pub fn global() -> &'static AppContext {
        APP_CONTEXT.get().expect(&os_responses::MommySuiteResponse::AppNotInit.to_string())
    }

    pub fn init(root_dir: PathBuf) {
        APP_CONTEXT.set(AppContext { root_dir })
            .expect(&os_responses::MommySuiteResponse::AppAlreadyInit.to_string());
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}


fn main() {
    phase_0_init();
    phase_1_credits();
    phase_2_waking_up();
    phase_3_core_feature_checks();
    phase_4_dir_check(AppContext::global().root_dir.as_path());
    phase_5_loading();
    phase_6_shell_handoff(AppContext::global().root_dir.as_path());
}

// ============================================================================
// BOOTLOADER PHASES
// ============================================================================

fn phase_0_init() {

    let root_dir = env::current_dir()
        .expect(&responses::MommyShellError::RootDirError.to_string());
    AppContext::init(root_dir);

}

fn phase_1_credits() {
    print!("{}", suite_constants::TERMINAL_CLEAR);
    io::stdout().flush().unwrap();

    println!("════════════════════════════════════════════════════════════════");
    println!("  │ Hello To my Custom Pseudo OS!");
    println!("  │ It is made by \"HiveMind\" to showcase my talents ^^.");
    println!("════════════════════════════════════════════════════════════════\n");
    thread::sleep(Duration::from_millis(suite_constants::MILLI_SLEEP_3000));
}

fn phase_2_waking_up() {
    print!("{}", suite_constants::TERMINAL_CLEAR);
    io::stdout().flush().unwrap();

    let thought1 = MommyBootloader::new("[SYS] ...head hurts...").with_delay(suite_constants::DELAY_MS_100);
    thought1.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_800));

    let thought2 = MommyBootloader::new("[SYS] ...where am I?").with_delay(suite_constants::DELAY_MS_150);
    thought2.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_500));

    let vitals = MommyBootloader::new("[SYS] Vitals checking:").with_delay(suite_constants::DELAY_MS_100);
    vitals.start(AnimationType::Heartbeat);
}

fn phase_3_core_feature_checks() {
    let exe_located = MommyBootloader::double_new("[SYS] Location at:", AppContext::global().root_dir.to_str().expect(&os_responses::MommySuiteResponse::RootNotFound.to_string())).with_delay(60);
    exe_located.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_400));
    let observation = MommyBootloader::new("[SYS] Vision adjusting. Scanning surroundings...").with_delay(suite_constants::DELAY_MS_60);
    observation.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_400));

    for exe in filesystem_manifest::CORE_EXE{
        let msg = format!("[0x08F{}] [SYS] Locating: {} ", suite_constants::REQ_CORE_FILES_NUM + filesystem_manifest::CORE_EXE.iter().position(|e| e.description == exe.description).unwrap(), exe.description);
        let check = MommyBootloader::new(&msg).with_delay(suite_constants::DELAY_MS_20);

        let actual_path = if cfg!(debug_assertions) {
            exe.build_path
        }
        else {
            exe.run_path };

        if Path::new(actual_path).exists() {
            check.start(AnimationType::Glitch("\x1B[32m[FOUND]\x1B[0m"));
        }
        else {
            check.start(AnimationType::Glitch("\x1B[31m[NOT FOUND]\x1B[0m"));
            thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_500));
            println!("\n\x1B[31m[KERNEL PANIC] Vital cognitive structure missing: {}\x1B[0m", actual_path);
            println!("\x1B[31m[KERNEL PANIC] Brain death imminent. Halting boot sequence.\x1B[0m\n");
            std::process::exit(1);
        }
    }

    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_600));
}

fn phase_4_dir_check(root_dir: &Path){
    let map_sys = MommyBootloader::new("[SYS] Validating physical boundaries...").with_delay(suite_constants::DELAY_MS_50);
    map_sys.start(AnimationType::Typewriter);

    for directory in filesystem_manifest::RequiredDirectory::ALL {
        let hex = directory.hex_code();
        let dir_name = directory.dir_name();
        let msg = format!("{} [SYS] Path: {} ", hex, dir_name);
        let check = MommyBootloader::new(&msg).with_delay(suite_constants::DELAY_MS_20);

        if Path::new(dir_name).exists() {
            check.start(AnimationType::Glitch("\x1B[32m[VERIFIED]\x1B[0m"));
        } else {
            check.start(AnimationType::Glitch("\x1B[31m[ABSENT]\x1B[0m"));
            std::process::exit(1);
        }
    }

    validate_mommy_config(root_dir);

    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_800));
    let scan_complete = MommyBootloader::new("[SYS] All critical systems present. Nowhere to run.").with_delay(suite_constants::DELAY_MS_80);
    scan_complete.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_1000));

}


fn phase_5_loading() {
    println!("\n\x1B[31mWelcome to MommySuite.");
    println!("Do not try to leave.\x1B[0m\n");
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_1200));

    let alert = MommyBootloader::new("\x1B[31m[WARNING] An entity has entered the terminal.\x1B[0m").with_delay(suite_constants::DELAY_MS_60);
    alert.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_1500));

    let observation = MommyBootloader::new("\x1B[31mShe sees that you are awake.\x1B[0m").with_delay(suite_constants::DELAY_MS_60);
    observation.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(suite_constants::DELAY_MS_1200));
}

fn phase_6_shell_handoff(root_dir: &Path) {
    let mut shell_command = Command::new(suite_constants::SHELL_EXE_PATH);

    let mut shell_process = shell_command
        .current_dir(root_dir)
        .env(suite_constants::OS_KEY_PATH, root_dir)
        .spawn()
        .expect(&os_responses::MommySuiteCoreResponse::ShellMissing.to_string());

    let status = shell_process.wait().expect(&os_responses::MommySuiteCoreResponse::ShellMissing.to_string());



    println!("\n[SYS] Shell terminated with status: {}", status);
}





//TODO. It needs to be part of the filesystem_manifest.rs. Ensuring that we can scale to more file checks.
fn validate_mommy_config(root_dir: &Path) {

    let properties_dir = root_dir.join(suite_constants::OS_CONFIG_PROPERTIES_DIR);
    let config_file = properties_dir.join(suite_constants::OS_CONFIG_MEMORY);
    let init_config = MommyBootloader::new("[0x09D0] [SYS] Loading neural configuration...").with_delay(suite_constants::DELAY_MS_20);

    if !properties_dir.exists() {
        fs::create_dir_all(&properties_dir).unwrap();
    }

    if !config_file.exists() {
        let mut settings = MommySettings::load(&root_dir);
        settings.user_name = String::new();
        settings.output_directory = String::new();

        match settings.save_user("") {
            Ok(_) => init_config.start(AnimationType::Glitch("\x1B[33m[ABSENT] -> [INITIALIZED]\x1B[0m")),
            Err(_) => {
                let empty_content = suite_constants::OS_CONFIG_MEMORY_CONTENT;
                match fs::write(&config_file, empty_content) {
                    Ok(_) => init_config.start(AnimationType::Glitch("\x1B[33m[MANUAL INITIALIZATION]\x1B[0m")),
                    Err(e) => init_config.start(AnimationType::Glitch(&format!("\x1B[31m[PERM ERROR: {}]\x1B[0m", e))),
                }
            }
        }
    } else {
        let _ = MommySettings::load(&root_dir);
        init_config.start(AnimationType::Glitch("\x1B[32m[LOADED]\x1B[0m"));
    }

}