use crate::api;
use crate::auth::utils::get_code_from_url;
use crate::auth::vault;
use crate::bots::bot_manager::BotState;

use log::error;
use std::error::Error;
use tauri::{AppHandle, Manager};
use tauri_plugin_oauth::{start_with_config, OauthConfig};

async fn handle_nightbot_login_flow(app: AppHandle, url: String) -> Result<(), Box<dyn Error>> {
    let authorization_code = get_code_from_url(url)?;
    let token = api::flow::get_nightbot_token(authorization_code).await?;

    // This is used to validate the token
    api::nightbot::get_channel(&token).await?;
    vault::store_token("Nightbot", &token)?;

    let state = app.state::<BotState>().clone();
    let mut bot_state = state.bot_manager.write().await;
    bot_state.set_bot("nightbot", &app).await?;

    Ok(())
}

#[tauri::command]
pub async fn start_nightbot_server(app: AppHandle) -> Result<(), String> {
    let config = OauthConfig {
        ports: Some(vec![8458]),
        ..OauthConfig::default()
    };

    let _ = start_with_config(config, move |url| {
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = handle_nightbot_login_flow(app_clone, url.to_string()).await {
                error!("[NIGHTBOT] Error while handling nightbot login flow : {}", e);
            }
        });
    });

    Ok(())
}
