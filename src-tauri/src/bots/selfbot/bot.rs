use crate::api::twitch;
use crate::states::{config::read_settings, context::read_context};

use log::error;
use reqwest::Result;
use tauri::AppHandle;

pub async fn announce(app: &AppHandle, message: String) -> Result<()> {
    let context = read_context(app).await;
    let channel_id = context.streamer.id.clone();

    let [token, moderator_id] = match read_settings(app).await.twitch_bot.as_str() {
        "custom" => [context.custom_bot_token.clone(), context.custom_bot.id.clone()],
        _ => [context.twitch_access_token.clone(), context.streamer.id.clone()],
    };

    if let Err(e) = twitch::send_announcement(&token, &channel_id, &moderator_id, message).await {
        error!("[TWITCH] Error while sending announcement : {}", e);
    }

    Ok(())
}
