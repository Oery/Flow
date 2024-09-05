use crate::api;
use crate::states::context::update_context;
use crate::states::structs::Streamer;

use futures::TryFutureExt;
use log::error;
use std::env;
use std::error::Error;
use tauri::{AppHandle, Manager};
use tauri_plugin_oauth::{start_with_config, OauthConfig};
use utils::get_code_from_url;

pub mod custom_bot_auth;
pub mod nightbot_auth;
pub mod twitch;
pub mod utils;
pub mod vault;

async fn login_with_token() -> anyhow::Result<(String, String)> {
    let token = vault::get_token("Twitch")?;

    match api::twitch::validate_token(&token).await {
        Ok(user_id) => Ok((user_id, token)),
        Err(_) => {
            let new_token = api::flow::refresh_twitch_token(token).await?;
            let user_id = api::twitch::validate_token(&new_token).await?;
            vault::store_token("Twitch", &new_token)?;
            Ok((user_id, new_token))
        }
    }
}

async fn load_streamer(token: &str, id: &str, app: &AppHandle) -> reqwest::Result<()> {
    let user = api::twitch::get_user(token, id).await?;
    let emotes = api::twitch::get_emotes(token, id).await?;

    let streamer = Streamer {
        display_name: user.display_name,
        id: user.id,
        avatar_url: user.profile_image_url,
        color: Default::default(),
        emotes,
    };

    update_context("streamer", serde_json::json!(streamer), app).await;
    update_context("twitch_access_token", serde_json::json!(token), app).await;

    Ok(())
}

fn start_oauth_server(app: AppHandle) -> Result<(), Box<dyn Error>> {
    let config = OauthConfig {
        ports: Some(vec![8457]),
        ..OauthConfig::default()
    };

    let _ = start_with_config(config, move |url| {
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = handle_login_flow(app, url).await {
                error!("[TWITCH] Error handling login flow: {:?}", e);
            }
        });
    });

    Ok(())
}

async fn handle_login_flow(app: AppHandle, url: String) -> Result<(), Box<dyn Error>> {
    let _ = &app.emit_all("logging-in", false);

    let authorization_code = get_code_from_url(url)?;
    let token = api::flow::get_twitch_token(authorization_code).await?;
    let _ = &app.emit_all("logging-in", true);

    api::twitch::validate_token(&token).await?;
    vault::store_token("Twitch", &token)?;

    Ok(())
}

#[tauri::command]
pub async fn load_app(app: AppHandle) -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if !args.contains(&"--background".to_string()) {
        if let Some(window) = app.get_window("main") {
            window.show().expect("Could not show the window");
        }
    }

    if let Ok((user_id, token)) = login_with_token().await {
        load_streamer(&token, &user_id, &app).map_err(|e| e.to_string()).await?;
        let _ = app.emit_all("loading-end", true);
        return Ok(());
    }

    let _ = app.emit_all("logging-in", false);
    start_oauth_server(app).map_err(|e| e.to_string())?;

    Ok(())
}
