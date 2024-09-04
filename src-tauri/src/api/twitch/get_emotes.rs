use http::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

// https://dev.twitch.tv/docs/api/reference/#get-channel-emotes

#[derive(Debug, Deserialize)]
pub struct Emote {
    id: String,
    name: String,
    format: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetEmotesResponse {
    data: Vec<Emote>,
    template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEmote {
    pub id: String,
    pub name: String,
    pub image_url: String,
}

pub async fn get_emotes(token: &str, streamer_id: &str) -> Result<Vec<FlowEmote>, Box<dyn std::error::Error>> {
    let url = reqwest::Url::parse_with_params("https://api.twitch.tv/helix/chat/emotes", &[("broadcaster_id", streamer_id)])?;

    let res = reqwest::Client::new()
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header("Client-Id", "cig4pc07b7bxo207x8158v58r1i5pf")
        .send()
        .await?
        .error_for_status()?;

    let json: GetEmotesResponse = res.json().await?;

    let emotes = json
        .data
        .iter()
        .map(|emote| {
            let emote_id = emote.id.clone();

            let format = match emote.format.iter().any(|format| format == "animated") {
                true => "animated",
                false => "static",
            };

            let image_url = json
                .template
                .replace("{{id}}", &emote_id)
                .replace("{{format}}", format)
                .replace("{{scale}}", "1.0")
                .replace("{{theme_mode}}", "light");

            FlowEmote {
                id: emote_id,
                name: emote.name.clone(),
                image_url,
            }
        })
        .collect::<Vec<FlowEmote>>();

    Ok(emotes)
}
