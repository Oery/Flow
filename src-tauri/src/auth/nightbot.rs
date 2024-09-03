use log::error;
use std::error::Error;

use tauri::AppHandle;
use tauri_plugin_oauth::{start_with_config, OauthConfig};

use super::{
    oauth::{trade_code_for_token, validate_token},
    oauth_services::Service,
    utils::parse_url,
    vault::store_token,
};

async fn handle_nightbot_login_flow(app: AppHandle, url: String) -> Result<(), Box<dyn Error>> {
    let code = parse_url(url)?;
    let token = trade_code_for_token(code, &Service::Nightbot).await?;

    validate_token(&token, &app, &Service::Nightbot).await?;
    store_token(&Service::Nightbot, &token)?;

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
