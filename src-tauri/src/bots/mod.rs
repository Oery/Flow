use bot_manager::BotState;
use std::error::Error;
use tauri::{AppHandle, Manager};

pub mod bot_manager;
pub mod nightbot;
pub mod selfbot;
pub mod twitch_bot;
pub mod wizebot;

pub async fn update_command(app: &AppHandle, command: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let state = app.state::<BotState>().clone();
    let bot_manager = state.bot_manager.read().await;
    bot_manager.get_bot().update_command(command, value).await
}
