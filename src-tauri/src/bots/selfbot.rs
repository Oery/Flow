use crate::api;
use crate::auth::vault;
use crate::bots::TwitchBot;
use crate::states::context::read_context;

use async_trait::async_trait;
use std::error::Error;
use tauri::AppHandle;

pub struct SelfBot {
    token: String,
    channel_id: String,
}

impl SelfBot {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            token: vault::get_token("Twitch")?,
            channel_id: String::new(),
        })
    }
}

#[async_trait]
impl TwitchBot for SelfBot {
    fn get_name(&self) -> &str {
        "selfbot"
    }

    async fn initialize(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let channel_id = read_context(app).await.streamer.id;
        self.channel_id = channel_id.clone();

        Ok(())
    }

    async fn update_command(&self, command: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
