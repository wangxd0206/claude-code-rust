//! Configuration Manager

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub theme: String,
    pub auto_save: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.anthropic.com".to_string(),
            model: "claude-3-opus-20240229".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            theme: "dark".to_string(),
            auto_save: true,
        }
    }
}

/// Configuration manager
#[derive(Debug)]
pub struct ConfigManager {
    config_dir: PathBuf,
    config_file: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("claude-code");

        let config_file = config_dir.join("config.toml");

        Self {
            config_dir,
            config_file,
        }
    }

    pub fn load(&self) -> Config {
        if self.config_file.exists() {
            match std::fs::read_to_string(&self.config_file) {
                Ok(content) => {
                    match toml::from_str(&content) {
                        Ok(config) => {
                            info!("Configuration loaded from {:?}", self.config_file);
                            return config;
                        }
                        Err(e) => {
                            warn!("Failed to parse config: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read config: {}", e);
                }
            }
        }

        Config::default()
    }

    pub fn save(&self, config: &Config) -> Result<(), String> {
        // Ensure config directory exists
        if let Err(e) = std::fs::create_dir_all(&self.config_dir) {
            return Err(format!("Failed to create config directory: {}", e));
        }

        match toml::to_string_pretty(config) {
            Ok(content) => {
                match std::fs::write(&self.config_file, content) {
                    Ok(_) => {
                        info!("Configuration saved to {:?}", self.config_file);
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to write config: {}", e)),
                }
            }
            Err(e) => Err(format!("Failed to serialize config: {}", e)),
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
