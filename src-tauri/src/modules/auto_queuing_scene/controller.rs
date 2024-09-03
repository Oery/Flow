use crate::states::config::Settings;

use log::error;
use tauri::{AppHandle, Manager};

use super::scene_manager::SceneState;

async fn show_screen(app: &AppHandle) {
    let state = app.state::<SceneState>().clone();
    let scene_manager = state.scene_manager.read().await;
    if let Err(e) = scene_manager.show_screen().await {
        error!("[OBS] Error while trying to show screen : {}", e);
    }
}

async fn hide_screen(app: &AppHandle, scene: &str) {
    let state = app.state::<SceneState>().clone();
    let mut scene_manager = state.scene_manager.write().await;
    if let Err(e) = scene_manager.hide_screen(scene).await {
        error!("[OBS] Error while trying to hide screen : {}", e);
    }
}

pub async fn set_screen(app: &AppHandle, settings: &Settings, should_show: bool) {
    if settings.scenes_enable {
        match should_show {
            true => show_screen(app).await,
            false => hide_screen(app, &settings.scenes_name).await,
        }
    }
}
