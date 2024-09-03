#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct CommandResponse {
    _total: i32,
    status: i32,
    commands: Vec<Command>,
}

async fn get_commands(token: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.nightbot.tv/1/commands");
    let res = reqwest::client::get(&url)?.text()?;
    Ok(res)
}
