use async_trait::async_trait;
use log::info;
use reqwest::Client;
use std::{collections::HashMap, error::Error};
use tokio::sync::Mutex;

use crate::auth::oauth_services::Service;
use crate::auth::vault::get_token;
use crate::bots::twitch_bot::TwitchBot;

pub struct Wizebot {
    api_key: String,
    client: Client,
    cache: Mutex<HashMap<String, String>>,
}

impl Wizebot {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let api_key = get_token(&Service::WizeBot)?;
        Ok(Self {
            api_key,
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

    fn announce(&self, _announcement: String) -> Result<(), Box<dyn Error>> {
        // TODO Feature not supported
        Ok(())
    }

    async fn initialize(&self) -> Result<(), Box<dyn Error>> {
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

        let url = format!("https://wapi.wizebot.tv/api/custom-data/{}/set/{}/{}", self.api_key, command, value);
        info!("[WIZEBOT] Command updated : {}, {}", command, value);

        self.client.post(url).send().await?.error_for_status()?;
        {
            let mut cache = self.cache.lock().await;
            cache.insert(command.to_string(), value.to_string());
        }

        Ok(())
    }
    // fn create_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
    //     Err("Wizebot cannot create command".into())
    // }

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

    // // fn get_commands(&self) -> Result<(), Box<dyn Error>> {}
    // fn update_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
    //     let url = format!(
    //         "https://wapi.wizebot.tv/api/custom-data/{}/set/{}/{}",
    //         self.api_key, command, value
    //     );
    //     let res = self.client.post(url).send()?.error_for_status()?;
    //     Ok(())
    // }
}
