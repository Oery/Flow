// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::states::config::SettingsState;
use bots::bot_manager;
use bots::bot_manager::BotState;
use log::info;
use states::{
    config::{get_flow_path, get_logs_path},
    context::AppState,
};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use window_shadows::set_shadow;

mod event_loop;
mod log_process;
mod log_reader;
mod mc_client;
mod windows_media;
use std::{env, error::Error, path::Path};

use crate::modules::auto_queuing_scene::scene_manager::SceneState;

mod auth;
mod bots;
mod commands;
mod modules;
mod states;
mod tailer;

use fern::colors::{Color, ColoredLevelConfig};

fn setup_flow_path() -> Result<(), Box<dyn Error>> {
    let flow_path = get_flow_path()?;
    if !Path::new(&flow_path).exists() {
        std::fs::create_dir_all(flow_path)?;
    }

    Ok(())
}

fn setup_logging() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new().info(Color::Green).warn(Color::Yellow).error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}]{}",
                chrono::Local::now().format("[%H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(get_logs_path().unwrap())?)
        .apply()?;

    Ok(())
}

fn main() {
    setup_flow_path().unwrap();
    setup_logging().unwrap();

    info!("[MAIN] Starting Flow...");

    let bot_manager_state = BotState::new();
    let settings_state = SettingsState::new();
    let app_state = AppState::default();
    let scene_state = SceneState::new();

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("Flow", "Flow").disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit Flow"));

    let tray = SystemTray::new().with_menu(tray_menu);

    let mut builder = tauri::Builder::default();
    builder = builder.setup(|app| {
        let win = app.get_window("main").unwrap();
        set_shadow(&win, true).unwrap();

        Ok(())
    });

    info!("[MAIN] Finished setup");

    // Autostart
    builder = builder.plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--background"])));
    info!("[MAIN] Autostart plugin loaded");

    // Single Instance
    // TODO: Fix single instance

    // Managed States
    builder = builder
        .manage(settings_state)
        .manage(app_state)
        .manage(bot_manager_state)
        .manage(scene_state);

    // Generate Commands
    builder = builder.invoke_handler(tauri::generate_handler![
        states::config::load_settings,
        states::config::update_setting,
        commands::alias::update_alias,
        commands::alias::delete_alias,
        event_loop::start_event_loop,
        auth::twitch::start_login_flow,
        auth::nightbot::start_nightbot_server,
        auth::twitch::log_out,
        states::context::load_context,
        bot_manager::get_current_bot,
        bot_manager::set_bot_token,
        bot_manager::set_current_bot,
        mc_client::get_mc_client,
        commands::preview::preview_command,
        log_process::update_command,
        log_process::hide_overlay,
        modules::auto_queuing_scene::scene_manager::get_scenes_list,
        modules::auto_queuing_scene::scene_manager::connect_to_obs,
    ]);
    info!("[MAIN] Tauri commands generated");

    // System Tray
    builder = builder.system_tray(tray).on_system_tray_event(|app, event| match event {
        SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    });
    info!("[MAIN] System Tray loaded");

    // Workaround for window resize insane lag on Windows
    builder = builder.on_window_event(|e| {
        if let WindowEvent::Resized(_) = e.event() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
    info!("[MAIN] Resize workaround loaded");

    info!("[MAIN] Starting Flow...");

    // Run
    builder.run(tauri::generate_context!()).expect("error while running flow");
}
