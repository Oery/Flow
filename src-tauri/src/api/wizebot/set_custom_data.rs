use reqwest::Result;

pub async fn set_custom_data(api_key: &str, command: &str, value: &str) -> Result<()> {
    let url = format!("https://wapi.wizebot.tv/api/custom-data/{}/set/{}/{}", api_key, command, value);
    reqwest::Client::new().post(url).send().await?.error_for_status()?;

    Ok(())
}
