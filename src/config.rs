#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub display: DisplayConfig,
    pub defaults: DefaultsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: String,
    pub skip_ssl_verify: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DisplayConfig {
    pub python_path: String,
    pub display_module: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultsConfig {
    pub default_user_id: usize,
    pub date_format: String,
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = "config/config.toml";
        
        // Check if config exists, if not, guide user
        if !std::path::Path::new(config_path).exists() {
            eprintln!("❌ Configuration file not found!");
            eprintln!("📝 Please create config/config.toml from config/config.toml.example");
            eprintln!("\nQuick setup:");
            eprintln!("  cp config/config.toml.example config/config.toml");
            eprintln!("  # Edit config/config.toml with your API details");
            std::process::exit(1);
        }
        
        let content = fs::read_to_string(config_path)
            .context("Failed to read config file")?;
        
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }
    
    /// Get Python interpreter path (use system default if not specified)
    pub fn python_interpreter(&self) -> String {
        if self.display.python_path.is_empty() {
            "python3".to_string()
        } else {
            self.display.python_path.clone()
        }
    }
}