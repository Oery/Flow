use crate::api;
use crate::states::{config::read_settings, context::read_context};

use bot_manager::BotState;
use log::error;
use std::error::Error;
use tauri::{AppHandle, Manager};

pub mod bot_manager;
mod custom;
mod nightbot;
mod selfbot;
mod twitch_bot;
mod wizebot;

pub use custom::*;
pub use nightbot::*;
pub use selfbot::*;
pub use twitch_bot::*;
pub use wizebot::*;

pub async fn update_command(app: &AppHandle, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let state = app.state::<BotState>().clone();
    let mut bot_manager = state.bot_manager.write().await;
    bot_manager.get_bot_mut().update_command(command, value).await?;
    Ok(())
}

pub async fn announce(app: &AppHandle, message: String) -> reqwest::Result<()> {
    let context = read_context(app).await;
    let channel_id = context.streamer.id.clone();

    let [token, moderator_id] = match read_settings(app).await.twitch_bot.as_str() {
        "custom" => [context.custom_bot_token.clone(), context.custom_bot_id.clone()],
        _ => [context.twitch_access_token.clone(), context.streamer.id.clone()],
    };

    if let Err(e) = api::twitch::send_announcement(&token, &channel_id, &moderator_id, message).await {
        error!("[TWITCH] Error while sending announcement : {}", e);
    }

    Ok(())
}
