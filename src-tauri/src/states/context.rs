use crate::states::structs::{IngameStatus, Streamer};

use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Context {
    pub streamer: Streamer,
    pub custom_bot_id: String,
    pub client: String,
    pub obs_status: String,
    pub bot_status: String,
    pub song_title: String,
    pub song_author: String,
    pub resource_pack_str: String,
    pub resource_packs_raw: Vec<String>,
    pub server_address: String,
    pub server_raw: String,
    pub ingame_status: IngameStatus,
    pub event_loop_running: bool,

    #[serde(skip_serializing)]
    pub twitch_access_token: String,
    #[serde(skip_serializing)]
    pub custom_bot_token: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            streamer: Streamer::default(),
            twitch_access_token: "".to_string(),
            custom_bot_token: "".to_string(),
            custom_bot_id: "".to_string(),
            client: "?".to_string(),
            obs_status: "Offline".to_string(),
            bot_status: "Offline".to_string(),
            song_title: "?".to_string(),
            song_author: "?".to_string(),
            resource_pack_str: "?".to_string(),
            resource_packs_raw: Vec::new(),
            server_address: "?".to_string(),
            server_raw: "?".to_string(),
            ingame_status: IngameStatus::Unknown,
            event_loop_running: false,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub context: Arc<RwLock<Context>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            context: Arc::new(RwLock::new(Context::default())),
        }
    }
}

#[tauri::command]
pub async fn load_context(app: AppHandle) -> Result<Context, String> {
    let mut ui_context = read_context(&app).await;
    ui_context.twitch_access_token = String::new();
    ui_context.custom_bot_token = String::new();

    Ok(ui_context)
}

pub async fn read_context(app: &AppHandle) -> Context {
    let state = app.state::<AppState>().clone();
    let data = state.context.read().await;
    data.clone()
}

pub async fn update_context(key: &str, value: serde_json::Value, app: &AppHandle) {
    let state = app.state::<AppState>().clone();
    let mut app_state = state.context.write().await;

    match key {
        "resource_pack_str" => app_state.resource_pack_str = value.as_str().unwrap_or("?").to_string(),
        "resource_packs_raw" => app_state.resource_packs_raw = serde_json::from_value(value.clone()).unwrap_or(Vec::new()),
        "server_address" => app_state.server_address = value.as_str().unwrap_or("?").to_string(),
        "server_raw" => app_state.server_raw = value.as_str().unwrap_or("?").to_string(),
        "client" => app_state.client = value.as_str().unwrap_or("?").to_string(),
        "bot_status" => app_state.bot_status = value.as_str().unwrap_or("?").to_string(),
        "custom_bot_id" => app_state.custom_bot_id = value.as_str().unwrap_or("?").to_string(),
        "obs_status" => app_state.obs_status = value.as_str().unwrap_or("?").to_string(),

        "song_title" => app_state.song_title = value.as_str().unwrap_or("?").to_string(),
        "song_author" => app_state.song_author = value.as_str().unwrap_or("?").to_string(),

        "ingame_status" => app_state.ingame_status = serde_json::from_value(value.clone()).unwrap_or(IngameStatus::Unknown),
        "streamer" => app_state.streamer = serde_json::from_value(value.clone()).unwrap_or(Streamer::default()),

        "twitch_access_token" => app_state.twitch_access_token = value.as_str().unwrap_or("").to_string(),
        _ => error!("[CONTEXT] Unknown key : {}", key),
    }

    let payload = serde_json::json!({ key: value });
    let _ = app.emit_all("update-context", payload);
}
