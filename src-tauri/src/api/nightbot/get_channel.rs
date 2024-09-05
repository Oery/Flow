use reqwest::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Channel {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct GetChannelResponse {
    pub channel: Channel,
}

pub async fn get_channel(token: &str) -> Result<Channel> {
    let response = reqwest::Client::new()
        .get("https://api.nightbot.tv/1/channel")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json::<GetChannelResponse>()
        .await?;

    Ok(response.channel)
}
