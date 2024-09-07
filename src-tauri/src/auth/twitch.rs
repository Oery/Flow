use crate::api;
use crate::auth::vault;
use crate::states::context::update_context;
use crate::states::structs::Streamer;

use log::error;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn log_out(app: AppHandle) -> Result<(), String> {
    let token = match vault::get_token("Twitch") {
        Ok(token) => token,
        Err(_) => {
            error!("[VAULT | TWITCH] Token not found");
            return Err("Token not found".to_string());
        }
    };

    if let Err(err) = api::twitch::revoke_token(token).await {
        error!("[TWITCH] Error while revoking token : {}", err);
        return Err(err.to_string());
    }

    if let Err(err) = vault::delete_token("Twitch") {
        error!("[TWITCH] Error while deleting token : {}", err);
        return Err(err.to_string());
    }

    update_context("twitch_access_token", serde_json::json!(""), &app).await;
    update_context("streamer", serde_json::json!(Streamer::default()), &app).await;

    // TODO: Stop Event Loop
    // TODO: Reload Context / Reload App

    Ok(())
}

#[tauri::command]
pub async fn close_custom_bot_auth(app: AppHandle) -> Result<(), String> {
    let window = app.get_window("twitch_bot_auth").unwrap();
    window.close().unwrap();
    Ok(())
}
