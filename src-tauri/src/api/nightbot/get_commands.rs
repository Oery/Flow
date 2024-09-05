use reqwest::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Command {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub name: String,
    pub message: String,
    #[serde(rename = "coolDown")]
    pub cool_down: i64,
    pub count: i64,
    #[serde(rename = "userLevel")]
    pub user_level: String,
}

#[derive(Debug, Deserialize)]
struct GetCommandsResponse {
    _total: i32,
    status: i32,
    commands: Vec<Command>,
}

pub async fn get_commands(token: &str) -> Result<Vec<Command>> {
    let data = reqwest::Client::new()
        .get("https://api.nightbot.tv/1/commands")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json::<GetCommandsResponse>()
        .await?;

    Ok(data.commands)
}
