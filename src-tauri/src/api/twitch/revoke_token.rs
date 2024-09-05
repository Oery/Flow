use crate::auth::vault::delete_token;

use std::error::Error;

// https://dev.twitch.tv/docs/authentication/revoke-tokens/

pub async fn revoke_token(token: String) -> Result<(), Box<dyn Error>> {
    let form = [("client_id", "cig4pc07b7bxo207x8158v58r1i5pf"), ("token", &token)];

    reqwest::Client::new()
        .post("https://id.twitch.tv/oauth2/revoke")
        .form(&form)
        .send()
        .await?
        .error_for_status()?;

    delete_token("Twitch")?;
    Ok(())
}
