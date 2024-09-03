use crate::{
    auth::{oauth_services::Service, vault::store_token},
    bots::{
        nightbot::bot::Nightbot,
        twitch_bot::{DefaultBot, TwitchBot},
        wizebot::bot::Wizebot,
    },
    states::{
        config::{Settings, SettingsState},
        context::update_context,
    },
};

use log::info;
use std::error::Error;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::State;
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
            "custom" => Err("Custom bot not supported".into()),
            _ => Ok(Box::new(DefaultBot {})),
        }
    }

    pub fn get_bot(&self) -> &dyn TwitchBot {
        self.current_bot.as_ref()
    }

    pub async fn set_bot(&mut self, name: &str, settings: &Settings) -> Result<(), String> {
        info!("[BOT] Setting bot to {}", name);
        let bot = self.new_bot(name, settings).map_err(|e| e.to_string())?;
        bot.initialize().await.map_err(|e| e.to_string())?;
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
pub async fn set_bot_token(
    bot: String,
    token: String,
    b: State<'_, BotState>,
    s: State<'_, SettingsState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let service = match bot.as_str() {
        "nightbot" => Service::Nightbot,
        "wizebot" => Service::WizeBot,
        _ => Service::Twitch,
    };

    store_token(&service, &token).map_err(|e| e.to_string())?;
    let mut bot_manager = b.bot_manager.write().await;
    let settings = s.settings.write().await.clone();

    info!("[BOT] Restarting Bot");

    match bot_manager.set_bot(bot.as_str(), &settings.clone()).await {
        Ok(_) => update_context("bot_status", serde_json::json!("Online"), &app_handle).await,
        Err(_) => update_context("bot_status", serde_json::json!("Offline"), &app_handle).await,
    }

    Ok(())
}

#[tauri::command]
pub async fn set_current_bot(
    b: State<'_, BotState>,
    s: State<'_, SettingsState>,
    app_handle: AppHandle,
    bot: String,
) -> Result<(), String> {
    if bot.is_empty() {
        return Ok(());
    }

    let mut bot_manager = b.bot_manager.write().await;
    let mut settings = s.settings.write().await.clone();

    settings.twitch_bot = bot.clone();

    match bot_manager.set_bot(bot.as_str(), &settings.clone()).await {
        Ok(_) => update_context("bot_status", serde_json::json!("Online"), &app_handle).await,
        Err(_) => update_context("bot_status", serde_json::json!("Offline"), &app_handle).await,
    };

    info!("[BOT] Bot set successfully");

    Ok(())
}
