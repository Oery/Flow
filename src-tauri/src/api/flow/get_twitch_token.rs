use reqwest::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GetTokenRequest {
    authorization_code: String,
}

#[derive(Debug, Deserialize)]
struct GetTokenResponse {
    token: String,
}

pub async fn get_twitch_token(authorization_code: String) -> Result<String> {
    let res = reqwest::Client::new()
        .post("https://api.oery.dev/auth/twitch/code")
        .json(&GetTokenRequest { authorization_code })
        .send()
        .await?
        .error_for_status()?;

    let json = res.json::<GetTokenResponse>().await?;
    Ok(json.token)
}
