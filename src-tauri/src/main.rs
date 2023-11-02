// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, RwLock};

use config::{load_settings_with_defaults, Settings};
use state::AppState;
use tauri::Manager;
use window_shadows::set_shadow;
mod config;
mod event_loop;
mod oauth;
mod state;
mod token;
mod windows_media;

fn main() {
    let settings = Arc::new(RwLock::new(
        load_settings_with_defaults().unwrap_or_default(),
    ));

    event_loop::get_loop(settings.clone());

    // let start_settings = {
    //     let locked_settings = settings.read().unwrap();
    //     locked_settings.clone()
    // };

    let app_state = AppState::default();

    // println!(
    //     "Hardware Acceleration : {}",
    //     start_settings.disable_hardware_acceleration
    // );

    let mut builder = tauri::Builder::default();
    builder = builder.setup(|app| {
        let win = app.get_window("main").unwrap();
        set_shadow(&win, true).unwrap();
        Ok(())
    });

    builder
        .manage(settings)
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            windows_media::get_current_song,
            config::load_settings,
            config::update_setting,
            oauth::start_login_flow,
            oauth::log_out,
            state::load_context,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
