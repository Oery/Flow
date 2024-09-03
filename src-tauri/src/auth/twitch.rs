use std::{env, error::Error};

use http::header::{AUTHORIZATION, CONTENT_TYPE};
use log::{debug, error, info};
use serde::Deserialize;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_oauth::{start_with_config, OauthConfig};

use crate::{
    auth::vault,
    states::{context::AppState, structs::Streamer},
};

use super::{
    oauth::{refresh_token, trade_code_for_token, validate_token},
    oauth_services::{Service, SERVICES},
    utils::parse_url,
    vault::{delete_token, get_token, store_token},
};

#[derive(Deserialize, Clone)]
struct GetStreamerInfoResponse {
    data: Vec<User>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
struct User {
    id: String,
    login: String,
    display_name: String,
    #[serde(rename = "type")]
    type_field: String,
    broadcaster_type: String,
    description: String,
    profile_image_url: String,
    offline_image_url: String,
    view_count: i32,
    created_at: String,
}

pub async fn get_streamer_info(token: &String, id: &String, app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let params = [("id", id)];

    let url = reqwest::Url::parse_with_params("https://api.twitch.tv/helix/users", &params)?;

    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header("Client-Id", "cig4pc07b7bxo207x8158v58r1i5pf")
        .send()
        .await?
        .error_for_status()?;

    let json: GetStreamerInfoResponse = response.json().await?;
    let twitch_streamer: User = json.data[0].clone();

    let streamer = Streamer {
        display_name: twitch_streamer.display_name,
        id: twitch_streamer.id,
        avatar_url: twitch_streamer.profile_image_url,
        color: Default::default(),
    };

    let app_state: State<AppState> = app.state();
    let mut locked_app_state = app_state.context.write().await;
    *locked_app_state = {
        let mut app_state = locked_app_state.clone();
        app_state.twitch_access_token = token.clone();
        app_state.streamer = streamer;
        let _ = app.emit_all("loading-end", true);
        let _ = app.emit_all("update-context", &app_state);
        app_state
    };

    Ok(())
}

#[tauri::command]
pub async fn start_login_flow(app: AppHandle) -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if !args.contains(&"--background".to_string()) {
        if let Some(window) = app.get_window("main") {
            window.show().expect("Could not show the window");
        }
    }

    info!("[TWITCH] Logging in...");
    try_login(&app).await.or_else(|_| start_server(app)).map_err(|e| e.to_string())
}

async fn try_login(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    debug!("[TWITCH] Getting token from vault...");
    let token = get_token(&Service::Twitch)?;

    info!("[TWITCH] Validating token...");
    if validate_token(&token, app, &Service::Twitch).await.is_err() {
        info!("[TWITCH] Token not valid, refreshing...");

        if let Err(e) = refresh_token(token, app, &Service::Twitch).await {
            error!("[TWITCH] Error while refreshing token : {}", e);
            vault::delete_token(SERVICES.twitch.vault)?;

            return Err(e);
        }
    }

    Ok(())
}

fn start_server(app: AppHandle) -> Result<(), String> {
    let _ = &app.emit_all("logging-in", false);

    // TODO: Send desktop notification if the app was started in background and the user is not logged in

    let config = OauthConfig {
        ports: Some(vec![8457]),
        ..OauthConfig::default()
    };

    let _ = start_with_config(config, move |url| {
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = handle_login_flow(app_clone, url.to_string()).await {
                error!("[TWITCH] Error handling login flow: {:?}", e);
            }
        });
    });

    Ok(())
}

async fn handle_login_flow(app: AppHandle, url: String) -> Result<(), Box<dyn Error>> {
    let _ = &app.emit_all("logging-in", false);

    let code = parse_url(url)?;
    let token = trade_code_for_token(code, &Service::Twitch).await?;
    let _ = &app.emit_all("logging-in", true);

    validate_token(&token, &app, &Service::Twitch).await?;
    store_token(&Service::Twitch, &token)?;

    Ok(())
}

#[tauri::command]
pub async fn log_out(app: AppHandle, app_state: State<'_, AppState>) -> Result<(), String> {
    let token = match get_token(&Service::Twitch) {
        Ok(token) => token,
        Err(_) => {
            error!("[VAULT | TWITCH] Token not found");
            return Err("Token not found".to_string());
        }
    };

    if let Err(err) = revoke_twitch_token(token).await {
        error!("[TWITCH] Error while revoking token : {}", err);
        return Err(err.to_string());
    }

    if let Err(err) = delete_token(SERVICES.twitch.vault) {
        error!("[TWITCH] Error while deleting token : {}", err);
        return Err(err.to_string());
    }

    let mut locked_app_state = app_state.context.write().await;
    *locked_app_state = {
        let mut app_state = locked_app_state.clone();
        app_state.twitch_access_token = "".to_string();
        app_state.streamer = Streamer::default();
        let _ = app.emit_all("update-context", &app_state);
        app_state
    };

    Ok(())
}

pub async fn revoke_twitch_token(token: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let params = [("client_id", "cig4pc07b7bxo207x8158v58r1i5pf"), ("token", &token)];

    client
        .post("https://id.twitch.tv/oauth2/revoke")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?
        .error_for_status()?;

    delete_token(SERVICES.twitch.vault)?;
    Ok(())
}
