use crate::api;
use crate::auth::vault::get_token;
use crate::bots::twitch_bot::TwitchBot;

use async_trait::async_trait;
use log::info;
use reqwest::Client;
use std::{collections::HashMap, error::Error};
use tauri::AppHandle;
use tokio::sync::Mutex;

pub struct Wizebot {
    api_key: String,
    client: Client,
    cache: Mutex<HashMap<String, String>>,
}

impl Wizebot {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            api_key: get_token("WizeBot")?,
            client: Client::new(),
            cache: Mutex::new(HashMap::new()),
        })
    }
}

#[async_trait]
impl TwitchBot for Wizebot {
    fn get_name(&self) -> &str {
        "wizebot"
    }

    async fn initialize(&mut self, _app: &AppHandle) -> Result<(), Box<dyn Error>> {
        let url = format!("https://wapi.wizebot.tv/api/custom-data/{}/get", self.api_key);
        let res = self.client.get(url).send().await?;

        // Forbidden means the URL exists therefore the key is valid
        if res.status() != reqwest::StatusCode::FORBIDDEN {
            res.error_for_status()?;
        }

        info!("[WIZEBOT] Initialized");

        Ok(())
    }

    async fn update_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
        {
            let cache = self.cache.lock().await;
            if cache.get(command) == Some(&value.to_string()) {
                return Ok(());
            }
        }

        api::wizebot::set_custom_data(&self.api_key, command, value).await?;
        info!("[WIZEBOT] Command updated : {}, {}", command, value);

        {
            let mut cache = self.cache.lock().await;
            cache.insert(command.to_string(), value.to_string());
        }

        Ok(())
    }

    // fn get_command(&self, command: &str) -> Result<(), Box<dyn Error>> {
    //     let url = format!(
    //         "https://wapi.wizebot.tv/api/custom-data/{}/get/{}",
    //         self.api_key, command
    //     );
    //     let res = self.client.get(url).send()?.error_for_status()?;
    //     let json = res.json()?;

    //     // Return command

    //     Ok(())
    // }
}
