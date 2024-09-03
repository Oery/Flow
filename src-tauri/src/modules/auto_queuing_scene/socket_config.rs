use anyhow::Result;
use log::{info, warn};
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::tailer::utils::format_path;

#[derive(Debug, Deserialize)]
pub struct SocketConfig {
    #[serde(rename = "server_enabled")]
    pub enabled: bool,
    #[serde(rename = "server_port")]
    pub port: u16,
    pub auth_required: bool,
    #[serde(rename = "server_password")]
    pub password: String,
}

impl SocketConfig {
    pub fn new() -> Self {
        Self {
            enabled: false,
            port: 0,
            auth_required: false,
            password: String::new(),
        }
    }

    pub fn load(path: &str) -> Result<Self> {
        info!("[OBS] Loading config...");

        let formatted_path = format_path(path);
        let json_config_path = Path::new(&formatted_path).join("plugin_config/obs-websocket/config.json");

        // Only works on obs-studio around < 30.0.1
        // Config was migrated to a json file in plugins-config/obs-websocket/config.json
        if json_config_path.exists() {
            info!("[OBS] JSON config file found, loading it");
            return Ok(serde_json::from_reader(File::open(json_config_path)?)?);
        }

        warn!("[OBS] Config file not found, trying legacy config. You should update your obs-studio version.");

        let legacy_config_path = Path::new(path).join("global.ini");
        Self::read_obs_config_file(&legacy_config_path)
    }

    pub fn read_obs_config_file(path: &std::path::PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let mut config = Self::new();

        for line in io::BufReader::new(file).lines() {
            let line = line?;

            match line.as_str() {
                value if value.starts_with("ServerEnabled=") => {
                    config.enabled = value.contains("true");
                }
                value if value.starts_with("ServerPort=") => {
                    config.port = match value.split_once('=').unwrap().1.parse::<u16>() {
                        Ok(value) => value,
                        Err(_) => continue,
                    };
                }
                value if value.starts_with("AuthRequired=") => {
                    config.auth_required = value.contains("true");
                }
                value if value.starts_with("ServerPassword=") => {
                    config.password = value.split_once('=').unwrap().1.to_string();
                }
                _ => continue,
            }
        }

        Ok(config)
    }
}
