use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use crate::constants;

pub struct MommySettings {
    pub output_directory: String,
    // Secret memory: Where is the notebook?
    config_file_path: PathBuf,
}

impl MommySettings {
    // 1. Load requires the ROOT (Home Base)
    pub fn load(root: &Path) -> Self {
        // Construct the absolute path: C:\Project\shell_properties\mommy_conf.memory
        let config_path = root.join("shell_properties").join("mommy_conf.memory");

        // Try to read
        let content = fs::read_to_string(&config_path).unwrap_or_default();

        // Default settings
        let mut settings = Self {
            output_directory: constants::DIR_OUTPUT.to_string(), // default "sandbox"
            config_file_path: config_path, // Remember the absolute path!
        };

        // Parse (simple key=value)
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                if key.trim() == "output" {
                    settings.output_directory = value.trim().to_string();
                }
            }
        }

        settings
    }

    // 2. Save uses the Absolute Path (Safe from 'cd')
    pub fn save(&self) -> io::Result<()> {
        let data = format!("output={}", self.output_directory);

        // Ensure folder exists
        if let Some(parent) = self.config_file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        fs::write(&self.config_file_path, data)
    }
}