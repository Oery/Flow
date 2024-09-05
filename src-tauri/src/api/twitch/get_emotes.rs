use reqwest::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};

// https://dev.twitch.tv/docs/api/reference/#get-channel-emotes

#[derive(Debug, Deserialize)]
pub struct TwitchEmote {
    id: String,
    name: String,
    format: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetEmotesResponse {
    data: Vec<TwitchEmote>,
    template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub name: String,
    pub image_url: String,
}

pub async fn get_emotes(token: &str, streamer_id: &str) -> Result<Vec<Emote>> {
    let url = Url::parse_with_params("https://api.twitch.tv/helix/chat/emotes", &[("broadcaster_id", streamer_id)]).unwrap();

    let res = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .header("Client-Id", "cig4pc07b7bxo207x8158v58r1i5pf")
        .send()
        .await?
        .error_for_status()?
        .json::<GetEmotesResponse>()
        .await?;

    let emotes = res
        .data
        .iter()
        .map(|emote| Emote::from_twitch_emote(emote, &res.template))
        .collect::<Vec<Emote>>();

    Ok(emotes)
}

impl Emote {
    pub fn from_twitch_emote(emote: &TwitchEmote, template: &str) -> Self {
        let format = match emote.format.iter().any(|format| format == "animated") {
            true => "animated",
            false => "static",
        };

        let image_url = template
            .replace("{{id}}", &emote.id)
            .replace("{{format}}", format)
            .replace("{{scale}}", "1.0")
            .replace("{{theme_mode}}", "light");

        Self {
            id: emote.id.clone(),
            name: emote.name.clone(),
            image_url,
        }
    }
}
