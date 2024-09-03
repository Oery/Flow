use reqwest::Result;
use serde::Serialize;
use tauri::AppHandle;

use crate::states::{config::read_settings, context::read_context};

// enum Color {
//     Blue,
//     Green,
//     Orange,
//     Purple,
//     Primary,
// }

#[derive(Serialize)]
struct Announcement {
    // color: Color,
    message: String,
}

pub async fn announce(app: &AppHandle, message: String) -> Result<()> {
    let context = read_context(app).await;
    let channel_id = context.streamer.id.clone();

    let [token, moderator_id] = match read_settings(app).await.twitch_bot.as_str() {
        "custom" => [context.custom_bot_token.clone(), context.custom_bot.id.clone()],
        _ => [context.twitch_access_token.clone(), context.streamer.id.clone()],
    };

    let url = format!(
        "https://api.twitch.tv/helix/chat/announcements?broadcaster_id={}&moderator_id={}",
        channel_id, moderator_id
    );

    reqwest::Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Client-Id", "cig4pc07b7bxo207x8158v58r1i5pf")
        .json(&Announcement { message })
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
