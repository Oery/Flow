use std::error::Error;

use crate::auth::oauth_services::SERVICES;
use crate::auth::vault::delete_token;

// https://dev.twitch.tv/docs/authentication/revoke-tokens/

pub async fn revoke_token(token: String) -> Result<(), Box<dyn Error>> {
    let form = [("client_id", "cig4pc07b7bxo207x8158v58r1i5pf"), ("token", &token)];

    reqwest::Client::new()
        .post("https://id.twitch.tv/oauth2/revoke")
        .form(&form)
        .send()
        .await?
        .error_for_status()?;

    delete_token(SERVICES.twitch.vault)?;
    Ok(())
}
