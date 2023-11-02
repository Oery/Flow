use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct Streamer {
    pub display_name: String,
    pub id: String,
    pub color: String,
    pub avatar_url: String,
}

impl Default for Streamer {
    fn default() -> Self {
        Self {
            display_name: "Streamer".to_string(),
            id: "".to_string(),
            color: "#6441a5".to_string(),
            avatar_url: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    pub streamer: Streamer,
    pub twitch_access_token: String,
    pub nightbot_access_token: String,
    pub client: String,
    pub obs_status: String,
    pub bot_status: String,
    pub song_title: String,
    pub song_author: String,
    pub resource_pack: String,
    pub server_address: String,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            streamer: Streamer::default(),
            twitch_access_token: "".to_string(),
            nightbot_access_token: "".to_string(),
            client: "".to_string(),
            obs_status: "Not connected".to_string(),
            bot_status: "Not connected".to_string(),
            song_title: "".to_string(),
            song_author: "".to_string(),
            resource_pack: "".to_string(),
            server_address: "".to_string(),
        }
    }
}
#[derive(Clone)]
pub struct AppState {
    pub data: Arc<RwLock<Data>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            data: Arc::new(RwLock::new(Data::default())),
        }
    }
}

#[tauri::command]
pub fn load_context(app_state: State<AppState>) -> Result<Data, String> {
    let locked_context = app_state.data.read().map_err(|e| e.to_string())?;
    Ok(locked_context.clone())
}
