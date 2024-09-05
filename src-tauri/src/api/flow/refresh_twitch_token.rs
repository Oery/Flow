use reqwest::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct RefreshTokenRequest {
    expired_token: String,
}

#[derive(Debug, Deserialize)]
struct RefreshTokenResponse {
    token: String,
}

pub async fn refresh_twitch_token(expired_token: String) -> Result<String> {
    let res = reqwest::Client::new()
        .post("https://api.oery.dev/auth/twitch/refresh")
        .json(&RefreshTokenRequest { expired_token })
        .send()
        .await?
        .error_for_status()?;

    let json = res.json::<RefreshTokenResponse>().await?;
    Ok(json.token)
}
