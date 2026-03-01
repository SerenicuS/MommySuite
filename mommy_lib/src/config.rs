use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use crate::constants;

pub struct MommySettings {
    pub output_directory: String,
    pub user_name: String,
    pub bin_exe: String,
    config_file_path: PathBuf,
}

impl MommySettings {
     pub fn load(root: &Path) -> Self {
        let config_path = root.join("mommy_properties").join("mommy_conf.memory");
        let content = fs::read_to_string(&config_path).unwrap_or_default();

        let mut settings = Self {
            output_directory: constants::DEF_DIR_OUTPUT.to_string(),
            user_name: String::new(),
            bin_exe: constants::BIN_EXE_DEF.to_string(),
            config_file_path: config_path,
        };

        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "output" => settings.output_directory = value.trim().to_string(),
                    "user" => settings.user_name = value.trim().to_string(),
                    "mommy_bin" => settings.bin_exe = value.trim().to_string(),
                    _ => {}
                }
            }
        }

        settings
    }

    pub fn save_path(&self) -> io::Result<()> {
        let data = format!(
            "output={}\nuser={}\nmommy_bin=",
            self.output_directory, self.user_name
        );

        if let Some(parent) = self.config_file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        fs::write(&self.config_file_path, data)
    }
    
    pub fn save_user(&self, new_user: &str) -> io::Result<()> {
        let content = fs::read_to_string(&self.config_file_path).unwrap_or_default();

        let mut updated = String::new();
        let mut user_found = false;

        for line in content.lines() {
            if let Some((key, _)) = line.split_once('=') {
                if key.trim() == "user" {
                    updated.push_str(&format!("user={}\n", new_user));
                    user_found = true;
                } else {
                    updated.push_str(&format!("{}\n", line));
                }
            } else {
                updated.push_str(&format!("{}\n", line));
            }
        }

        if !user_found {
            updated.push_str(&format!("user={}\n", new_user));
        }

        fs::write(&self.config_file_path, updated.trim())
    }

    pub fn username_does_not_exist(&self) -> bool{
        self.user_name.trim().is_empty()
    }
}
