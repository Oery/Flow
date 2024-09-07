use crate::api;
use crate::auth::vault;
use crate::bots::TwitchBot;
use crate::states::context::{read_context, update_context};

use async_trait::async_trait;
use log::error;
use std::collections::HashMap;
use std::error::Error;
use tauri::AppHandle;

pub struct CustomBot {
    token: String,
    bot_id: String,
    channel_id: String,
    commands: HashMap<String, String>,
}

impl CustomBot {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            token: vault::get_token("CustomBot")?,
            bot_id: String::new(),
            channel_id: String::new(),
            commands: HashMap::new(),
        })
    }
}

#[async_trait]
impl TwitchBot for CustomBot {
    fn get_name(&self) -> &str {
        "custom"
    }

    async fn initialize(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let context = read_context(app).await;
        self.channel_id = context.streamer.id.clone();

        if let Ok(user_id) = api::twitch::validate_token(&self.token).await {
            update_context("custom_bot_id", serde_json::json!(user_id), app).await;
            self.bot_id = user_id;
            return Ok(());
        }

        // Token is invalid, refresh it
        let token = api::flow::refresh_twitch_token(self.token.to_string()).await?;
        let user_id = api::twitch::validate_token(&token).await?;
        vault::store_token("CustomBot", &token)?;
        update_context("custom_bot_id", serde_json::json!(user_id), app).await;
        self.bot_id = user_id;
        self.token = token;

        Ok(())
    }

    async fn update_command(&mut self, command: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        error!("[CUSTOMBOT] Command updating not implemented");
        self.commands.insert(command.to_string(), value.to_string());
        Ok(())
    }
}
