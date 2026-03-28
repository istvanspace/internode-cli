use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::CliError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_url")]
    pub api_url: String,
    #[serde(default)]
    pub api_key: String,
}

fn default_api_url() -> String {
    "https://agents.api.internode.ai/".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: default_api_url(),
            api_key: String::new(),
        }
    }
}

pub fn config_dir() -> Result<PathBuf, CliError> {
    let base = dirs::config_dir()
        .ok_or_else(|| CliError::Internal("Cannot determine config directory".into()))?;
    Ok(base.join("internode"))
}

pub fn ensure_config_dir() -> Result<PathBuf, CliError> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn config_path() -> Result<PathBuf, CliError> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn load_config() -> Config {
    let path = match config_path() {
        Ok(p) => p,
        Err(_) => return Config::default(),
    };
    match fs::read_to_string(&path) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

pub fn save_config(config: &Config) -> Result<(), CliError> {
    let dir = ensure_config_dir()?;
    let path = dir.join("config.toml");
    let content = toml::to_string_pretty(config)
        .map_err(|e| CliError::Internal(format!("Failed to serialize config: {e}")))?;
    fs::write(&path, content)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

pub fn require_api_key(config: &Config) -> Result<(), CliError> {
    if config.api_key.is_empty() {
        return Err(CliError::Auth(
            "No API key configured. Run `internode configure` first.".into(),
        ));
    }
    Ok(())
}
