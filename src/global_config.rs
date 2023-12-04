use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use dirs;

#[derive(Deserialize, Serialize, Default)]
pub struct CLIConfig {
    pub default_command: String,
}

impl CLIConfig {
    pub fn default_path() -> PathBuf {
        dirs::home_dir().expect("Home directory not found")
            .join(".dev_quick_setup")
            .join("cli_config.json")
    }

    pub fn load_or_init() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::default_path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        if !path.exists() {
            let config = CLIConfig {
                default_command: "devsetup".to_string(),
            };
            config.save(&path)?;
            Ok(config)
        } else {
            Self::load(&path)
        }
    }

    fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        let config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(path, config_str)?;
        Ok(())
    }
}

