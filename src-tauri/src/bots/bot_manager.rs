use crate::auth::vault;
use crate::bots::{CustomBot, DefaultBot, Nightbot, SelfBot, TwitchBot, Wizebot};
use crate::states::config::{read_settings, Settings, SettingsState};
use crate::states::context::update_context;

use log::info;
use std::error::Error;
use std::sync::Arc;
use tauri::State;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

pub struct BotManager {
    current_bot: Box<dyn TwitchBot>,
}

impl BotManager {
    pub fn new() -> Self {
        Self {
            current_bot: Box::new(DefaultBot {}),
        }
    }

    fn new_bot(&self, name: &str, settings: &Settings) -> Result<Box<dyn TwitchBot>, Box<dyn Error>> {
        match name {
            "nightbot" => Ok(Box::new(Nightbot::new(settings)?)),
            "wizebot" => Ok(Box::new(Wizebot::new()?)),
            "custom" => Ok(Box::new(CustomBot::new()?)),
            "self" => Ok(Box::new(SelfBot::new()?)),
            _ => Ok(Box::new(DefaultBot {})),
        }
    }

    pub fn get_bot(&self) -> &dyn TwitchBot {
        self.current_bot.as_ref()
    }

    pub async fn set_bot(&mut self, name: &str, app: &AppHandle) -> Result<(), String> {
        let settings = read_settings(app).await;
        info!("[BOT] Setting bot to {}", name);
        let mut bot = self.new_bot(name, &settings).map_err(|e| e.to_string())?;
        bot.initialize(app).await.map_err(|e| e.to_string())?;
        update_context("bot_status", serde_json::json!("Online"), app).await;
        self.current_bot = bot;
        Ok(())
    }
}

#[derive(Clone)]
pub struct BotState {
    pub bot_manager: Arc<RwLock<BotManager>>,
}

impl BotState {
    pub fn new() -> Self {
        Self {
            bot_manager: Arc::new(RwLock::new(BotManager::new())),
        }
    }
}

#[tauri::command]
pub async fn get_current_bot(state: State<'_, BotState>) -> Result<String, ()> {
    let bot_manager = state.bot_manager.read().await;
    Ok(bot_manager.get_bot().get_name().to_string())
}

#[tauri::command]
pub async fn set_bot_token(bot: String, token: String, app: AppHandle) -> Result<(), String> {
    let resource = match bot.as_str() {
        "nightbot" => "Nightbot",
        "wizebot" => "WizeBot",
        "custom" => "CustomBot",
        _ => "Twitch",
    };

    vault::store_token(resource, &token).map_err(|e| e.to_string())?;

    let b = app.state::<BotState>().clone();
    let mut bot_manager = b.bot_manager.write().await;

    info!("[BOT] Restarting Bot");

    match bot_manager.set_bot(&bot, &app).await {
        Ok(_) => update_context("bot_status", serde_json::json!("Online"), &app).await,
        Err(_) => update_context("bot_status", serde_json::json!("Offline"), &app).await,
    }

    Ok(())
}

#[tauri::command]
pub async fn set_current_bot(app: AppHandle, bot: String) -> Result<(), String> {
    if bot.is_empty() {
        return Ok(());
    }

    let b = app.state::<BotState>().clone();
    let mut bot_manager = b.bot_manager.write().await;

    let s = app.state::<SettingsState>().clone();
    let mut settings = s.settings.write().await.clone();

    settings.twitch_bot = bot.clone();

    match bot_manager.set_bot(bot.as_str(), &app).await {
        Ok(_) => update_context("bot_status", serde_json::json!("Online"), &app).await,
        Err(_) => update_context("bot_status", serde_json::json!("Offline"), &app).await,
    };

    info!("[BOT] Bot set successfully");

    Ok(())
}
