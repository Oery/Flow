use reqwest::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GetCustomDataResponse {
    success: bool,
    key: String,
    val: String,
    code: i32,
    cache_timestamps: i32,
}

pub async fn get_custom_data(api_key: &str, key: &str) -> Result<String> {
    let res = reqwest::Client::new()
        .get(format!("https://wapi.wizebot.tv/api/custom-data/{}/get/{}", api_key, key))
        .send()
        .await?
        .error_for_status()?
        .json::<GetCustomDataResponse>()
        .await?;

    Ok(res.val)
}
