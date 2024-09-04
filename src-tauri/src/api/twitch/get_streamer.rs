use http::header::AUTHORIZATION;
use serde::Deserialize;
use std::error::Error;

// https://dev.twitch.tv/docs/api/reference/#get-users

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub profile_image_url: String,
}

#[derive(Deserialize, Debug, Clone)]
struct GetUserResponse {
    data: Vec<User>,
}

pub async fn get_user(token: &String, id: &String) -> Result<User, Box<dyn Error>> {
    let url = reqwest::Url::parse_with_params("https://api.twitch.tv/helix/users", &[("id", id)])?;

    let res = reqwest::Client::new()
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header("Client-Id", "cig4pc07b7bxo207x8158v58r1i5pf")
        .send()
        .await?
        .error_for_status()?;

    let json: GetUserResponse = res.json().await?;
    let user = json.data[0].clone();

    Ok(user)
}
