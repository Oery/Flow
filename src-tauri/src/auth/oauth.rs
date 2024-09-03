use http::header::AUTHORIZATION;
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tauri::AppHandle;

use crate::{auth::twitch::get_streamer_info, states::context::update_context};

use super::{oauth_services::Service, vault::store_token};

#[derive(Deserialize)]
struct ValidateTokenResponse {
    // client_id: String,
    // login: String,
    // scopes: Vec<String>,
    user_id: String,
    // expires_in: i64,
}

#[derive(Deserialize)]
struct RefreshTokenResponse {
    token: String,
}

#[derive(Serialize)]
struct GetTokenRequest {
    authorization_code: String,
}

#[derive(Deserialize)]
struct GetTokenResponse {
    token: String,
}

#[derive(Serialize)]
struct RefreshTokenRequest {
    expired_token: String,
}

pub async fn trade_code_for_token(authorization_code: String, service: &Service) -> Result<String, Box<dyn Error>> {
    let url = service.get_code_url()?;

    let client = reqwest::Client::new();
    let request_body = GetTokenRequest { authorization_code };
    let response = client.post(url).json(&request_body).send().await?.error_for_status()?;

    let token_response: GetTokenResponse = response.json().await?;
    Ok(token_response.token)
}

pub async fn refresh_token(expired_token: String, app: &AppHandle, service: &Service) -> Result<(), Box<dyn Error>> {
    info!("[{}] Refreshing Token...", service);

    let body = reqwest::Client::new()
        .post(service.get_refresh_url()?)
        .json(&RefreshTokenRequest { expired_token })
        .send()
        .await?
        .error_for_status()?
        .json::<RefreshTokenResponse>()
        .await?;

    store_token(service, &body.token)?;
    validate_token(&body.token, app, service).await?;

    Ok(())
}

pub async fn validate_token(token: &String, app: &AppHandle, service: &Service) -> Result<(), Box<dyn Error>> {
    info!("[{}] Validating Token...", service);

    let response = reqwest::Client::new()
        .get(service.get_validate_url()?)
        .header(AUTHORIZATION, service.get_header(token)?)
        .send()
        .await?
        .error_for_status()?;

    store_token(service, token)?;

    if service == &Service::Twitch {
        let json: ValidateTokenResponse = response.json().await?;
        get_streamer_info(token, &json.user_id, app).await?;
    }

    if service == &Service::Nightbot {
        update_context("bot_status", serde_json::json!("Online"), app).await;
    }

    info!("[{}] Token validated", service);

    Ok(())
}
