use serde::Serialize;
use std::error::Error;

// https://dev.twitch.tv/docs/api/reference/#send-chat-announcement

#[derive(Serialize)]
struct Announcement {
    message: String,
}

pub async fn send_announcement(token: &str, channel_id: &str, moderator_id: &str, message: String) -> Result<(), Box<dyn Error>> {
    let url = reqwest::Url::parse_with_params(
        "https://api.twitch.tv/helix/chat/announcements",
        &[("broadcaster_id", channel_id), ("moderator_id", moderator_id)],
    )?;

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
