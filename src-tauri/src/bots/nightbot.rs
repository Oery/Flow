use crate::api;
use crate::api::flow::refresh_nightbot_token;
use crate::auth::vault::{get_token, store_token};
use crate::bots::twitch_bot::TwitchBot;
use crate::states::config::Settings;

use async_trait::async_trait;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Command {
    _id: String,
    name: String,
    message: String,
    count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct CommandResponse {
    _total: i32,
    status: i32,
    commands: Vec<Command>,
}

pub struct Nightbot {
    token: String,
    prefix: String,
    client: Client,
}

#[async_trait]
impl TwitchBot for Nightbot {
    fn get_name(&self) -> &str {
        "nightbot"
    }

    async fn initialize(&mut self, _app: &AppHandle) -> Result<(), Box<dyn Error>> {
        if api::nightbot::get_channel(&self.token).await.is_err() {
            let new_token = refresh_nightbot_token(self.token.to_string()).await?;
            store_token("Nightbot", &new_token)?;
            self.token = new_token;
        }
        Ok(())
    }

    async fn update_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let command = self.prefix.clone() + command;
        let commands = api::nightbot::get_commands(&self.token).await?;

        info!("[NIGHTBOT] Updating command: {}", command);

        let mut target_id = String::new();
        for c in commands {
            if c.name == command {
                target_id = c.id.clone();
                break;
            }
        }

        if target_id.is_empty() {
            error!("[NIGHTBOT] Command not found: {}", command);
            return Err("Command not found".into());
        }

        api::nightbot::update_command(&target_id, value, &self.token).await?;

        Ok(())
    }
}

impl Nightbot {
    pub fn new(settings: &Settings) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            token: get_token("Nightbot")?,
            prefix: settings.bot_prefix.clone(),
            // commands: Vec::new(),
            client: Client::new(),
        })
    }

    // async fn create_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
    //     let url = "https://api.nightbot.tv/1/commands";
    //     let res = self.client
    //         .post(url)
    //         .bearer_auth(&self.token)
    //         .json({
    //             name: command.to_string(),
    //             message: value.to_string(),
    //             cooldown: 0,
    //             count: 0,
    //             user_level: "".to_string(),
    //             created_at: "".to_string(),
    //             updated_at: "".to_string(),
    //         })
    //         .send()
    //         .await?
    //         .error_for_status()?;

    //     let command_response: CommandResponse = res.json().await?;
}
