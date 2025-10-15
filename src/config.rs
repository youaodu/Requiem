use crate::i18n::Language;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// UI language
    pub language: Language,
    /// Directory to save collections and requests
    pub save_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        let save_directory = dirs::home_dir()
            .map(|p| p.join(".requiem").to_string_lossy().to_string())
            .unwrap_or_else(|| ".requiem".to_string());

        Self {
            language: Language::default(),
            save_directory,
        }
    }
}

impl Config {
    /// Load configuration from file, or create default if not exists
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str::<Config>(&content) {
                    Ok(config) => {
                        debug!("Loaded config from: {:?}", config_path);
                        return config;
                    }
                    Err(e) => {
                        warn!("Failed to parse config file: {}. Using default.", e);
                    }
                },
                Err(e) => {
                    warn!("Failed to read config file: {}. Using default.", e);
                }
            }
        } else {
            debug!("Config file not found, creating default at: {:?}", config_path);
        }

        let config = Self::default();
        // Try to save default config
        if let Err(e) = config.save() {
            warn!("Failed to save default config: {}", e);
        }
        config
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        debug!("Saved config to: {:?}", config_path);

        Ok(())
    }

    /// Get the path to the config file
    fn config_path() -> PathBuf {
        // Try XDG config directory first, then fall back to home directory
        let config_dir = dirs::config_dir()
            .map(|p| p.join("requiem"))
            .or_else(|| dirs::home_dir().map(|p| p.join(".config").join("requiem")))
            .unwrap_or_else(|| PathBuf::from(".config/requiem"));

        config_dir.join("config.toml")
    }

    /// Update language and save
    pub fn set_language(&mut self, language: Language) -> Result<(), Box<dyn std::error::Error>> {
        self.language = language;
        self.save()
    }

    /// Update save directory and save
    pub fn set_save_directory(&mut self, directory: String) -> Result<(), Box<dyn std::error::Error>> {
        self.save_directory = directory;
        self.save()
    }
}
