use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::{error, info};

#[derive(Debug)]
pub struct SocketConfig {
    pub enabled: bool,
    pub port: u16,
    pub auth_required: bool,
    pub password: String,
}

impl SocketConfig {
    pub fn fetch_socket_config(&mut self) -> io::Result<()> {
        let app_data = match env::var("APPDATA") {
            Ok(path) => path,
            Err(_) => {
                error!("[OBS] APPDATA not found");
                return Err(io::Error::new(io::ErrorKind::NotFound, "APPDATA not found"));
            }
        };

        let obs_config_path = Path::new(&app_data).join("obs-studio/global.ini");
        let file = File::open(obs_config_path)?;

        info!("[OBS] Loading config...");

        for line in io::BufReader::new(file).lines() {
            let line = line?;

            match line.as_str() {
                value if value.starts_with("ServerEnabled=") => {
                    self.enabled = value.contains("true");
                }
                value if value.starts_with("ServerPort=") => {
                    self.port = match value.split_once('=').unwrap().1.parse::<u16>() {
                        Ok(value) => value,
                        Err(_) => continue,
                    };
                }
                value if value.starts_with("AuthRequired=") => {
                    self.auth_required = value.contains("true");
                }
                value if value.starts_with("ServerPassword=") => {
                    self.password = value.split_once('=').unwrap().1.to_string();
                }
                _ => continue,
            }
        }

        info!("[OBS] Config loaded");

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            enabled: false,
            port: 0,
            auth_required: false,
            password: String::new(),
        }
    }
}
