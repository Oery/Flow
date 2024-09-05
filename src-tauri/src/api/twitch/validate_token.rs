use reqwest::Result;
use serde::Deserialize;

// https://dev.twitch.tv/docs/authentication/validate-tokens/

#[derive(Deserialize, Debug)]
struct ValidateTokenResponse {
    user_id: String,
}

pub async fn validate_token(token: &str) -> Result<String> {
    let res = reqwest::Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json::<ValidateTokenResponse>()
        .await?;

    Ok(res.user_id)
}
