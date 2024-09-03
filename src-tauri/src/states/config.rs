use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alias {
    pub alias: String,
    pub hidden: bool,
}

impl Default for Alias {
    fn default() -> Self {
        Self {
            alias: "".to_string(),
            hidden: false,
        }
    }
}

impl Alias {
    pub fn new(alias: String, hidden: bool) -> Self {
        Self { alias, hidden }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aliases {
    pub packs: HashMap<String, Alias>,
    pub servers: HashMap<String, Alias>,
}

impl Aliases {
    pub fn new() -> Self {
        Self {
            packs: HashMap::new(),
            servers: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub enable: bool,
    pub streaming_only: bool,
    pub streaming_mc_only: bool,
    pub mc_client: String,
    pub custom_logs_path: String,
    pub twitch_bot: String,
    pub bot_prefix: String,
    pub disable_hardware_acceleration: bool,
    pub start_with_windows: bool,
    pub language: String,

    pub scenes_enable: bool,
    pub scenes_name: String,
    pub scenes_delay: u64,
    pub scenes_hide_in_lobby: bool,
    pub scenes_toggle_after_game_end: bool,
    pub scenes_auto_obs_config: bool,

    pub pack_enable: bool,
    pub pack_command: String,
    pub pack_command_text: String,
    pub pack_hide_overlay: bool,
    pub pack_announcements_enable: bool,
    pub pack_announcements_text: String,

    pub server_enable: bool,
    pub server_command: String,
    pub server_command_text: String,
    pub server_announcements_enable: bool,
    pub server_announcements_text: String,

    pub music_enable: bool,
    pub music_ignore_twitch: bool,
    pub music_command: String,
    pub music_command_text: String,
    pub music_announcements_enable: bool,
    pub music_announce_text: String,

    pub stats_enable: bool,
    pub stats_labels_folder: String,
    pub stats_username: String,

    pub aliases: Aliases,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enable: true,
            streaming_only: true,
            streaming_mc_only: true,
            mc_client: "Vanilla / Forge".to_string(),
            custom_logs_path: "default_path".to_string(),
            twitch_bot: "Nightbot".to_string(),
            bot_prefix: "!".to_string(),
            disable_hardware_acceleration: false,
            start_with_windows: false,
            language: "auto".to_string(),

            scenes_enable: false,
            scenes_name: "Queuing Scene".to_string(),
            scenes_hide_in_lobby: false,
            scenes_delay: 3,
            scenes_toggle_after_game_end: true,
            scenes_auto_obs_config: true,

            pack_enable: false,
            pack_command: "!pack".to_string(),
            pack_command_text: "Resource Pack : {pack}".to_string(),
            pack_hide_overlay: false,
            pack_announcements_enable: true,
            pack_announcements_text: "Resource Pack : {pack}".to_string(),

            server_enable: false,
            server_command: "!ip".to_string(),
            server_command_text: "IP : {server}".to_string(),
            server_announcements_enable: true,
            server_announcements_text: "IP : {server}".to_string(),

            music_enable: false,
            music_ignore_twitch: true,
            music_command: "!music".to_string(),
            music_command_text: "🎵 : {title} - {artist}".to_string(),
            music_announcements_enable: true,
            music_announce_text: "🎵 Now playing : {title} - {artist} 🎵".to_string(),

            stats_enable: false,
            stats_labels_folder: "./labels".to_string(),
            stats_username: "".to_string(),

            aliases: Aliases::new(),
        }
    }
}

fn deep_merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), Value::Object(b)) => {
            for (k, v) in b {
                deep_merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            if a.is_null() {
                *a = b.clone();
            }
        }
    }
}

pub fn save_settings_to_file(settings: &Settings) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(settings)?;
    let file_path = get_config_path()?;

    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_settings_with_defaults() -> Result<Settings, Box<dyn Error>> {
    let default_settings = Settings::default();
    let default_value: Value = serde_json::to_value(default_settings)?;

    let file_path = get_config_path()?;

    let mut loaded_value = if Path::new(&file_path).exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents)?
    } else {
        Value::Object(Default::default())
    };

    deep_merge(&mut loaded_value, &default_value);

    let merged_settings: Settings = serde_json::from_value(loaded_value)?;
    Ok(merged_settings)
}

fn get_config_path() -> Result<String, Box<dyn Error>> {
    let app_data = env::var("APPDATA")?;
    Ok(format!("{}\\Flow - Streaming Utility\\settings.json", app_data))
}

pub fn get_logs_path() -> Result<String, Box<dyn Error>> {
    let app_data = env::var("APPDATA")?;
    Ok(format!("{}\\Flow - Streaming Utility\\flow.log", app_data))
}

pub fn get_flow_path() -> Result<String, Box<dyn Error>> {
    let app_data = env::var("APPDATA")?;
    Ok(format!("{}\\Flow - Streaming Utility", app_data))
}

macro_rules! check_default {
    ($settings:expr, $field:ident, $default:expr) => {
        if $settings.$field == $default.$field {
            String::new()
        } else {
            $settings.$field.clone()
        }
    };
}

macro_rules! replace_empty_with_default {
    ($settings:expr, $field:ident, $default:expr) => {
        if $settings.$field.is_empty() {
            $settings.$field = $default.$field.clone();
        }
    };
}

impl Settings {
    fn to_frontend_format(&self) -> Self {
        let default = Settings::default();
        let mut settings = self.clone();

        settings.mc_client = check_default!(self, mc_client, default);
        settings.custom_logs_path = check_default!(self, custom_logs_path, default);
        settings.twitch_bot = check_default!(self, twitch_bot, default);
        settings.scenes_name = check_default!(self, scenes_name, default);
        settings.pack_command_text = check_default!(self, pack_command_text, default);
        settings.pack_announcements_text = check_default!(self, pack_announcements_text, default);
        settings.server_command_text = check_default!(self, server_command_text, default);
        settings.server_announcements_text = check_default!(self, server_announcements_text, default);
        settings.music_command_text = check_default!(self, music_command_text, default);
        settings.music_announce_text = check_default!(self, music_announce_text, default);

        settings
    }

    fn from_frontend_format(&mut self) {
        let default = Settings::default();

        replace_empty_with_default!(self, mc_client, default);
        replace_empty_with_default!(self, custom_logs_path, default);
        replace_empty_with_default!(self, twitch_bot, default);
        replace_empty_with_default!(self, scenes_name, default);
        replace_empty_with_default!(self, pack_command_text, default);
        replace_empty_with_default!(self, pack_announcements_text, default);
        replace_empty_with_default!(self, server_command_text, default);
        replace_empty_with_default!(self, server_announcements_text, default);
        replace_empty_with_default!(self, music_command_text, default);
        replace_empty_with_default!(self, music_announce_text, default);
    }
}

#[derive(Clone)]
pub struct SettingsState {
    pub settings: Arc<RwLock<Settings>>,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(RwLock::new(load_settings_with_defaults().unwrap_or_default())),
        }
    }
}

#[tauri::command]
pub async fn load_settings(state: State<'_, SettingsState>) -> Result<Settings, ()> {
    let locked_settings = state.settings.read().await;
    Ok(locked_settings.clone().to_frontend_format())
}

#[tauri::command]
pub async fn update_setting(key: String, value: serde_json::Value, state: State<'_, SettingsState>) -> Result<(), String> {
    let mut locked_settings = state.settings.write().await;

    if !key.contains("password") {
        debug!("[CONFIG] Updating: {} = {}", key, value);
    }

    let mut settings_json: serde_json::Value = serde_json::to_value(&*locked_settings).unwrap();
    settings_json[key] = value;

    *locked_settings = serde_json::from_value(settings_json).unwrap_or_else(|e| {
        error!("[CONFIG] Failed to update: {}", e);
        locked_settings.clone()
    });

    locked_settings.from_frontend_format();

    if let Err(e) = save_settings_to_file(&locked_settings) {
        error!("[CONFIG] Failed to save: {}", e);
        return Err(e.to_string());
    }

    Ok(())
}

pub async fn read_settings(app: &AppHandle) -> Settings {
    let state = app.state::<SettingsState>().clone();
    let settings = state.settings.read().await;
    settings.clone()
}
