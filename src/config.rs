// config.rs

use serde::{Deserialize, Serialize};
use std::{fs};

#[derive(Deserialize, Serialize)]
pub struct FullConfig {
    pub languages: Vec<String>,
    pub text_editors: Vec<String>,
    pub frameworks: Vec<String>,
    pub networking_tools: Vec<String>,
    pub utilities: Vec<String>,
    pub extras: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub basic: Vec<String>,
    pub full: FullConfig,
    pub customized: Vec<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        let config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        println!("{}", config_str);
        fs::write(path, config_str)?;
        println!("Config saved to {}", path);
        Ok(())
    }

    pub fn update_customized(&mut self, tools: &[String]) {
        println!("Updating customized tools: {:?}", tools);
        self.customized = tools.to_vec();
    }

}
