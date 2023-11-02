extern crate reqwest;
extern crate serde;
extern crate serde_json;
use crate::state::{AppState, Streamer};

use crate::token::{
    delete_token, get_token, revoke_twitch_token, save_token, validate_twitch_token,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;
use tauri::{self, AppHandle, Manager, State};
use tauri_plugin_oauth::{start_with_config, OauthConfig};
use url::Url;

#[derive(Serialize)]
struct GetTokenRequest {
    authorization_code: String,
}

#[derive(Deserialize)]
struct GetTokenResponse {
    token: String,
}

#[tauri::command]
pub fn start_login_flow(app: AppHandle) -> Result<(), String> {
    try_login(&app)
        .or_else(|_| start_server(app))
        .map_err(|e| e.to_string())
}

fn try_login(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let token = get_token("TwitchOAuthToken")?;
    validate_twitch_token(&token, app)?;
    Ok(())
}

#[tauri::command]
pub fn log_out(app: AppHandle, app_state: State<AppState>) -> Result<(), String> {
    let token = get_token("TwitchOAuthToken").unwrap();
    revoke_twitch_token(token).unwrap();
    delete_token("TwitchOAuthToken".to_string()).unwrap();

    let mut locked_app_state = app_state.data.write().unwrap();
    *locked_app_state = {
        let mut app_state = locked_app_state.clone();
        app_state.twitch_access_token = "".to_string();
        app_state.streamer = Streamer::default();
        let _ = app.emit_all("update-context", &app_state);
        app_state
    };

    Ok(())
}

fn start_server(app: AppHandle) -> Result<(), String> {
    let _ = &app.emit_all("logging-in", false);

    let config = OauthConfig {
        ports: Some(vec![8457]),
        ..OauthConfig::default()
    };

    start_with_config(config, move |url| {
        let code = parse_url(url).unwrap();
        let token = trade_code_for_token(code).unwrap();

        let _ = &app.emit_all("logging-in", true);
        validate_twitch_token(&token, &app).unwrap();
        save_token("TwitchOAuthToken", &token).unwrap();
    })
    .map_err(|err| err.to_string())?;

    Ok(())
}

fn parse_url(url_str: String) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(&url_str)?;
    match url.query_pairs().find(|(key, _)| key == "code") {
        Some((_, value)) => Ok(value.into_owned()),
        None => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "No code found",
        ))),
    }
}

fn trade_code_for_token(authorization_code: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let request_body = GetTokenRequest { authorization_code };

    let response = client
        .post("https://api.oery.dev/oauth/twitch/code")
        .json(&request_body)
        .send()
        .unwrap();

    if !response.status().is_success() {
        let error_text = response.text().unwrap();
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, error_text)));
    }

    let token_response: GetTokenResponse = response.json().unwrap();
    Ok(token_response.token)
}
