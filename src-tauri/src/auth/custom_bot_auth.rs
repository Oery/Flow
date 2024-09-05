use crate::api;
use crate::auth::utils::get_code_from_url;
use crate::auth::utils::open_custom_auth_window;
use crate::auth::vault;

use log::error;
use std::error::Error;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_oauth::start_with_config;
use tauri_plugin_oauth::OauthConfig;

async fn handle_login(app: AppHandle, url: String) -> Result<(), Box<dyn Error>> {
    if let Some(window) = app.get_window("twitch_bot_auth") {
        let _ = window.close();
    }
    let authorization_code = get_code_from_url(url)?;
    let token = api::flow::get_twitch_token(authorization_code).await?;

    api::twitch::validate_token(&token).await?;
    vault::store_token("CustomBot", &token)?;

    Ok(())
}

fn start_server(app: AppHandle) -> Result<(), Box<dyn Error>> {
    let config = OauthConfig {
        ports: Some(vec![8457]),
        ..OauthConfig::default()
    };

    let _ = start_with_config(config, move |url| {
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = handle_login(app_clone, url).await {
                error!("[CUSTOMBOT] Error while handling custom bot login flow : {}", e);
            }
        });
    });

    Ok(())
}

#[tauri::command]
pub async fn start_custom_bot_auth(app: tauri::AppHandle) -> Result<(), String> {
    open_custom_auth_window(&app);
    start_server(app).map_err(|e| e.to_string())?;
    Ok(())
}
