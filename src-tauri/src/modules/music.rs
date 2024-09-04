use crate::{
    bots::{bot_manager::BotState, selfbot},
    states::config::SettingsState,
    windows_media::{get_current_song, is_media_paused},
};
use log::{error, info};
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
        let music_info = get_current_song().await?;
        let title = music_info.title;
        let artist = music_info.artist;

        let settings = {
            let state = app.state::<SettingsState>().clone();
            let locked_settings = state.settings.read().await;
            locked_settings.clone()
        };

        if title == self.current_title || is_media_paused().await? || (title.is_empty() && artist.is_empty()) {
            return Ok(());
        }

        if settings.music_ignore_twitch && title.ends_with(" - Twitch") {
            return Ok(());
        }

        let command = settings.music_command_text.replace("{title}", &title).replace("{artist}", &artist);

        let result = {
            let bot_state = app.state::<BotState>().clone();
            let bot_manager = bot_state.bot_manager.read().await;

            if settings.music_announcements_enable {
                let announcement = settings.music_announce_text.replace("{title}", &title).replace("{artist}", &artist);
                let _ = selfbot::bot::announce(app, announcement).await;
            }

            bot_manager.get_bot().update_command("music", &command).await
        };

        match result {
            Ok(_) => info!("[DJ] Music command updated"),
            Err(e) => error!("[DJ] Error while updating music command : {}", e),
        }

        self.current_title = title;
        Ok(())
    }
}
