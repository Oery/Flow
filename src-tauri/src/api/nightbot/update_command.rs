use reqwest::Result;
use serde::{Deserialize, Serialize};

// https://api-docs.nightbot.tv/#edit-custom-command-by-id

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommandUpdateBuilder {
    count: Option<i32>,
    message: Option<String>,
}

impl CommandUpdateBuilder {
    fn new() -> Self {
        Self {
            count: None,
            message: None,
        }
    }
}

pub async fn update_command(command_id: &str, value: &str, token: &str) -> Result<()> {
    let mut payload = CommandUpdateBuilder::new();
    payload.message = Some(value.to_string());

    reqwest::Client::new()
        .put(format!("https://api.nightbot.tv/1/commands/{}", command_id))
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
