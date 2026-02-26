use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::{fs, thread};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::env;
use mommy_lib::config::MommySettings;
use mommy_lib::responses;

fn get_jitter(max: u64) -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as u64) % max
}

struct MommyBootloader {
    message: String,
    base_delay_ms: u64,
}

impl MommyBootloader {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            base_delay_ms: 50,
        }
    }

    fn with_delay(mut self, delay_ms: u64) -> Self {
        self.base_delay_ms = delay_ms;
        self
    }

   fn start(&self, animation_type: AnimationType) {
        match animation_type {
            AnimationType::Typewriter => self.typewriter_animation(),
            AnimationType::Heartbeat => self.heartbeat_animation(),
            AnimationType::Glitch(end_tag) => self.glitch_animation(end_tag),
        }
    }

    // Prints character by character like a person slowly waking up
    fn typewriter_animation(&self) {
        for c in self.message.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();

            // Add random jitter so the typing feels human/broken
            let jitter = get_jitter(100);
            thread::sleep(Duration::from_millis(self.base_delay_ms + jitter));
        }
        println!(); // Move to next line
    }

    // A flatline that occasionally spikes
    fn heartbeat_animation(&self) {
        print!("{} ", self.message);
        let frames = vec!["_ _ _ _", "_ _ ^ _", "_ / \\ _", "_ _ _ _", "_ _ _ _", "_ _ _ _"];

        for i in 0..15 {
            print!("\r{} {}", self.message, frames[i % frames.len()]);
            io::stdout().flush().unwrap();

            // Heartbeats aren't perfectly steady when you're panicked
            let jitter = get_jitter(150);
            thread::sleep(Duration::from_millis(self.base_delay_ms + jitter));
        }
        print!("\r\x1B[2K{} [CRITICAL]\n", self.message);
    }

    // Rapidly flashes garbage characters before stabilizing
    fn glitch_animation(&self, end_tag: &str) {
        let garbage = vec!["0xFA", "NULL", "ERR", "####", "0x00"];
        for i in 0..20 {
            print!("\r{} {}", self.message, garbage[i % garbage.len()]);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(30));
        }
        // Prints whatever tag we pass in!
        print!("\r\x1B[2K{} {}\n", self.message, end_tag);
    }
}

enum AnimationType<'a> {
    Typewriter,
    Heartbeat,
    Glitch(&'a str), // Now it takes the ending word dynamically!
}


fn main() {
    phase_1_credits();
    phase_2_waking_up();
    phase_3_file_checks();
    let root_dir = phase_4_map_the_cage();
    phase_5_the_hijack();
    phase_6_shell_handoff(&root_dir);
}

// ============================================================================
// BOOTLOADER PHASES
// ============================================================================

fn phase_1_credits() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("════════════════════════════════════════════════════════════════");
    println!("  │ Hello To my Custom Pseudo OS!");
    println!("  │ It is made by \"HiveMind\" to showcase my talents ^^.");
    println!("════════════════════════════════════════════════════════════════\n");

    thread::sleep(Duration::from_millis(3000));
}

fn phase_2_waking_up() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    let thought1 = MommyBootloader::new("[SYS] ...head hurts...").with_delay(100);
    thought1.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(800));

    let thought2 = MommyBootloader::new("[SYS] ...where am I?").with_delay(150);
    thought2.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(500));

    let vitals = MommyBootloader::new("[SYS] Vitals checking:").with_delay(100);
    vitals.start(AnimationType::Heartbeat);
}

fn phase_3_file_checks() {
    let observation = MommyBootloader::new("[0x08F4] [SYS] Vision adjusting. Scanning surroundings...").with_delay(60);
    observation.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(400));

    let vital_systems = vec![
        ("mommy_shell.exe (communication terminal)", "target/debug/mommy_shell.exe", "./mommy_shell.exe"),
        ("mommy_lang.exe (cognitive core)", "target/debug/mommy_lang.exe", "./mommy_lang.exe"),
        ("mommy_editor.exe (instruction interface)", "mommy_editor/mommy_editor.exe", "mommy_editor.exe"),
    ];

    for (i, (name, debug_path, release_path)) in vital_systems.iter().enumerate() {
        let msg = format!("[0x08F{}] [SYS] Locating: {} ", 5 + i, name);
        let check = MommyBootloader::new(&msg).with_delay(20);

        let actual_path = if cfg!(debug_assertions) { *debug_path } else { *release_path };

        // REALITY CHECK
        if Path::new(actual_path).exists() {
            check.start(AnimationType::Glitch("[FOUND]"));
        } else {
            check.start(AnimationType::Glitch("\x1B[31m[NOT FOUND]\x1B[0m"));
            thread::sleep(Duration::from_millis(500));
            println!("\n\x1B[31m[KERNEL PANIC] Vital cognitive structure missing: {}\x1B[0m", actual_path);
            println!("\x1B[31m[KERNEL PANIC] Brain death imminent. Halting boot sequence.\x1B[0m\n");
            std::process::exit(1);
        }
    }
    thread::sleep(Duration::from_millis(600));
}

fn phase_4_map_the_cage() -> std::path::PathBuf {
    let map_sys = MommyBootloader::new("[0x09A2] [SYS] Validating physical boundaries...").with_delay(50);
    map_sys.start(AnimationType::Typewriter);

    let directories = vec![
        ("[0x09B0]", "mommy_brain"),
        ("[0x09B4]", "mommy_trash"),
        ("[0x09C2]", "mommy_properties"),
        ("[0x09C8]", "mommy_memory"),
        ("[0x09CC]", "sandbox"), // Ensure sandbox is in the mapping list
    ];

    for (hex, dir_name) in directories {
        let msg = format!("{} [SYS] Path: {} ", hex, dir_name);
        let check = MommyBootloader::new(&msg).with_delay(20);

        if Path::new(dir_name).exists() {
            check.start(AnimationType::Glitch("[VERIFIED]"));
        } else {
            fs::create_dir_all(dir_name).unwrap();
            check.start(AnimationType::Glitch("\x1B[33m[ABSENT] -> [CONSTRUCTED]\x1B[0m"));
        }
    }


    let root_dir = env::current_dir().expect(&responses::MommyShellError::RootDirError.to_string());

    // 1. Manually build the exact absolute path to the file
    let properties_dir = root_dir.join("mommy_properties");
    let config_file = properties_dir.join("mommy_conf.memory");

    let init_config = MommyBootloader::new("[0x09D0] [SYS] Loading neural configuration...").with_delay(20);

    // 2. Ensure the directory exists one more time (Safety)
    if !properties_dir.exists() {
        std::fs::create_dir_all(&properties_dir).unwrap();
    }

    if !config_file.exists() {
        // 3. Load the settings struct
        let mut settings = MommySettings::load(&root_dir);

        // 4. Force empty fields as requested
        settings.user_name = String::new();
        settings.output_directory = String::new();

        // 5. Explicitly check if we can write a dummy file here to test permissions
        // If save_user() fails, it's because the library doesn't know about 'properties_dir'
        match settings.save_user("") {
            Ok(_) => init_config.start(AnimationType::Glitch("\x1B[33m[ABSENT] -> [INITIALIZED]\x1B[0m")),
            Err(_) => {
                // DEBUG: If it still fails, let's manually force the file creation
                let empty_content = "output=\nuser=\n";
                match std::fs::write(&config_file, empty_content) {
                    Ok(_) => init_config.start(AnimationType::Glitch("\x1B[33m[MANUAL INITIALIZATION]\x1B[0m")),
                    Err(e) => init_config.start(AnimationType::Glitch(&format!("\x1B[31m[PERM ERROR: {}]\x1B[0m", e))),
                }
            }
        }
    } else {
        let _ = MommySettings::load(&root_dir);
        init_config.start(AnimationType::Glitch("[LOADED]"));
    }

    thread::sleep(Duration::from_millis(800));
    let scan_complete = MommyBootloader::new("[0x0A10] [SYS] All critical systems present. Nowhere to run.").with_delay(80);
    scan_complete.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(1000));

    root_dir
}

fn phase_5_the_hijack() {
    println!("\n\x1B[31mWelcome to MommySuite.");
    println!("Do not try to leave.\x1B[0m\n");
    thread::sleep(Duration::from_millis(1200));

    let alert = MommyBootloader::new("\x1B[31m[WARNING] An entity has entered the terminal.\x1B[0m").with_delay(60);
    alert.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(1500));

    let observation = MommyBootloader::new("\x1B[31mShe sees that you are awake.\x1B[0m").with_delay(150);
    observation.start(AnimationType::Typewriter);
    thread::sleep(Duration::from_millis(1200));
}

fn phase_6_shell_handoff(root_dir: &Path) {
    let mut shell_command = if cfg!(debug_assertions) {
        Command::new("target/debug/mommy_shell.exe")
    } else {
        Command::new("./mommy_shell.exe")
    };

    // Inject the root directory directly into the shell's memory environment
    let mut shell_process = shell_command
        .current_dir(root_dir)
        .env("MOMMY_ROOT_DIR", root_dir)
        .spawn()
        .expect("CRITICAL SYSTEM FAILURE: mommy_shell missing or corrupted.");

    let status = shell_process.wait().unwrap();
    println!("\n[SYS] Shell terminated with status: {}", status);
}