use crate::api;
use crate::auth::vault;
use crate::states::{context::AppState, structs::Streamer};

use log::error;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn log_out(app: AppHandle, app_state: State<'_, AppState>) -> Result<(), String> {
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

#[tauri::command]
pub async fn close_custom_bot_auth(app: AppHandle) -> Result<(), String> {
    let window = app.get_window("twitch_bot_auth").unwrap();
    window.close().unwrap();
    Ok(())
}
