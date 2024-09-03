use crate::auth::{oauth_services::Service, vault::get_token};
use crate::bots::twitch_bot::TwitchBot;
use crate::states::config::Settings;

use async_trait::async_trait;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Command {
    _id: String,
    name: String,
    message: String,
    // cool_down: i32,
    count: i32,
    // user_level: String,
    // created_at: String,
    // updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct CommandResponse {
    _total: i32,
    status: i32,
    commands: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommandUpdateBuilder {
    // cool_down: Option<i32>,
    count: Option<i32>,
    message: Option<String>,
    // user_level: Option<String>,
}

impl CommandUpdateBuilder {
    fn new() -> Self {
        Self {
            // cool_down: None,
            count: None,
            message: None,
            // user_level: None,
        }
    }
}

// struct CommandCreateBuilder {
//     name: String,
//     message: String,
//     cool_down: i32,
//     user_level: String,
// }

// impl CommandCreateBuilder {
//     fn new(name: &str, message: &str, cool_down: i32, count: i32, user_level: &str) -> Self {
//         Self {
//             name: name.to_string(),
//             message: message.to_string(),
//             cool_down: 0,
//             user_level: "everyone".to_string(),
//         }
//     }
// }

pub struct Nightbot {
    token: String,
    prefix: String,
    // commands: Vec<Command>,
    client: Client,
}

#[async_trait]
impl TwitchBot for Nightbot {
    fn get_name(&self) -> &str {
        "nightbot"
    }

    async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        self.client
            .get("https://api.nightbot.tv/1/channel")
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn update_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let command = self.prefix.clone() + command;
        let commands = self.get_commands().await?;

        info!("[NIGHTBOT] Updating command: {}", command);

        let mut target_id = String::new();
        for c in commands {
            if c.name == command {
                target_id = c._id.clone();
                break;
            }
        }

        if target_id.is_empty() {
            error!("[NIGHTBOT] Command not found: {}", command);
            return Err("Command not found".into());
        }

        let mut payload = CommandUpdateBuilder::new();
        payload.message = Some(value.into());

        let url = format!("https://api.nightbot.tv/1/commands/{}", target_id);
        self.client
            .put(url)
            .bearer_auth(&self.token)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

impl Nightbot {
    pub fn new(settings: &Settings) -> Result<Self, Box<dyn Error>> {
        let token = get_token(&Service::Nightbot)?;
        Ok(Self {
            token,
            prefix: settings.bot_prefix.clone(),
            // commands: Vec::new(),
            client: Client::new(),
        })
    }

    async fn get_commands(&self) -> Result<Vec<Command>, Box<dyn Error>> {
        let url = "https://api.nightbot.tv/1/commands";
        let res = self.client.get(url).bearer_auth(&self.token).send().await?.error_for_status()?;
        let command_response: CommandResponse = res.json().await?;

        Ok(command_response.commands)
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
