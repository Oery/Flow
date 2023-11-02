use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub enable: bool,
    pub streaming_only: bool,
    pub streaming_mc_only: bool,
    pub mc_client: String,
    pub custom_logs_path: String,
    pub twitch_bot: String,
    pub disable_hardware_acceleration: bool,

    pub scenes_enable: bool,
    pub scenes_name: String,
    pub scenes_toggle_after_game_end: bool,
    pub scenes_manual_obs_config: bool,

    pub pack_enable: bool,
    pub pack_command_text: String,
    pub pack_announcements_enable: bool,
    pub pack_announcements_text: String,

    pub server_enable: bool,
    pub server_command_text: String,
    pub server_announcements_enable: bool,
    pub server_announcements_text: String,

    pub music_enable: bool,
    pub music_command_text: String,
    pub music_announcements_enable: bool,
    pub music_announce_text: String,
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
            disable_hardware_acceleration: false,

            scenes_enable: false,
            scenes_name: "Queuing Scene".to_string(),
            scenes_toggle_after_game_end: true,
            scenes_manual_obs_config: false,

            pack_enable: false,
            pack_command_text: "Resource Pack : {pack}".to_string(),
            pack_announcements_enable: true,
            pack_announcements_text: "Resource Pack : {pack}".to_string(),

            server_enable: false,
            server_command_text: "Server IP : {server}".to_string(),
            server_announcements_enable: true,
            server_announcements_text: "Server IP : {server}".to_string(),

            music_enable: false,
            music_command_text: "🎵 : {music} - {artist}".to_string(),
            music_announcements_enable: true,
            music_announce_text: "🎵 Now playing : {music} - {artist} 🎵".to_string(),
        }
    }
}

fn deep_merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
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

fn save_settings_to_file(settings: &Settings) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(settings).unwrap();
    let mut file = File::create("C:/Users/Oery/Documents/settings.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_settings_with_defaults() -> Result<Settings, Box<dyn std::error::Error>> {
    let default_settings = Settings::default();
    let default_value: Value = serde_json::to_value(&default_settings)?;

    let mut loaded_value = if Path::new("C:/Users/Oery/Documents/settings.json").exists() {
        let mut file = File::open("C:/Users/Oery/Documents/settings.json")?;
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
        settings.server_announcements_text =
            check_default!(self, server_announcements_text, default);
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

#[tauri::command]
pub fn load_settings(
    settings: tauri::State<'_, Arc<RwLock<Settings>>>,
) -> Result<Settings, String> {
    let start = Instant::now();
    println!("Attempting to lock settings...");
    let locked_settings = settings.read().map_err(|e| e.to_string())?;
    println!("Settings locked successfully.");
    let duration = start.elapsed();
    println!("Time elapsed for this iteration is: {:?}", duration);
    Ok(locked_settings.clone().to_frontend_format())
}

#[tauri::command]
pub fn update_setting(
    key: String,
    value: serde_json::Value,
    settings: tauri::State<'_, Arc<RwLock<Settings>>>,
) {
    let start = Instant::now();
    let mut locked_settings = settings.write().unwrap();

    println!("Updating setting: Key = {}, Value = {}", key, value);

    // Convert the current settings to a mutable JSON object
    let mut settings_json: serde_json::Value = serde_json::to_value(&*locked_settings).unwrap();
    settings_json[key] = value;

    // Convert the JSON object back to the Settings struct
    *locked_settings = serde_json::from_value(settings_json).unwrap_or_else(|e| {
        eprintln!("Failed to update setting: {}", e);
        locked_settings.clone()
    });

    locked_settings.from_frontend_format();

    save_settings_to_file(&*locked_settings).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed for setting update is: {:?}", duration);
}
