use crate::bots;
use crate::states::config::read_settings;
use crate::{
    api::windows::{get_current_song, is_media_paused},
    bots::bot_manager::BotState,
    states::{config::SettingsState, context::update_context},
};

use log::error;
use std::error::Error;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Clone)]
pub struct Music {
    current_title: String,
}

impl Music {
    pub fn new() -> Self {
        Self {
            current_title: String::new(),
        }
    }

    pub async fn update(&mut self, app: &AppHandle) -> Result<(), Box<dyn Error>> {
        let settings = read_settings(app).await;
        let music_info = get_current_song().await?;
        let mut title = music_info.title;
        let mut artist = music_info.artist;

        if title == self.current_title || is_media_paused().await? || (title.is_empty() && artist.is_empty()) {
            return Ok(());
        }

        if title.ends_with(" - Twitch") {
            if settings.music_ignore_twitch {
                return Ok(());
            }

            title = title.replace(" - Twitch", "");
            artist = "Twitch".to_string();
        }

        let command = settings.music_command_text.replace("{title}", &title).replace("{artist}", &artist);

        let bot_state = app.state::<BotState>().clone();
        let bot_manager = bot_state.bot_manager.read().await;

        if let Err(e) = bot_manager.get_bot().update_command("music", &command).await {
            error!("[DJ] Error while updating music command : {}", e);
        }

        if settings.music_announcements_enable {
            let announcement = settings.music_announce_text.replace("{title}", &title).replace("{artist}", &artist);
            if let Err(e) = bots::announce(app, announcement).await {
                error!("[DJ] Error while announcing music : {}", e);
            }
        }

        update_context("song_title", serde_json::json!(title), app).await;
        update_context("song_author", serde_json::json!(artist), app).await;

        self.current_title = title;
        Ok(())
    }
}
