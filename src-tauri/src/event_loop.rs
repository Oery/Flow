use crate::log_reader::LogReader;
use crate::mc_client::{get_current_client, Client, CLIENT_NOT_FOUND};
use crate::modules::auto_queuing_scene::scene_manager::SceneState;
use crate::modules::music::Music;
use crate::states::config::read_settings;
use crate::states::context::{update_context, AppState};
use log::{debug, error, info};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use tokio::time;

const MUSIC_COOLDOWN_IN_MS: u128 = 5_000;

pub async fn connect_to_obs(app: &AppHandle) {
    info!("[OBS] Trying to connect to OBS...");

    let state = app.state::<SceneState>().clone();
    let mut scene_manager = state.scene_manager.write().await;

    if scene_manager.client.is_some() {
        return;
    }

    let settings = read_settings(app).await;
    let task = time::timeout(Duration::from_secs(1), scene_manager.connect(&settings)).await;

    let status = match task {
        Ok(Ok(_)) => "Online",
        Ok(Err(e)) => {
            error!("[OBS] Error while connecting to OBS : {}", e);
            "Offline"
        }
        Err(_timeout) => {
            error!("[OBS] Timeout while connecting to OBS");
            "Offline"
        }
    };

    if status == "Online" {
        info!("[OBS] WS Connected");
    }

    update_context("obs_status", serde_json::json!(status), app).await;
}

pub async fn update_client(client: &Client, app: &AppHandle, log_reader: &Arc<Mutex<LogReader>>) {
    update_context("client", serde_json::json!(&client.display_name), app).await;
    info!("[MC] Client changed -> {:?}", client.display_name);

    let mut log_reader_lock = log_reader.lock().await;
    let _ = log_reader_lock.read_init(app, client.path).await;
}

pub async fn get_loop(app: &AppHandle) {
    debug!("[EVENT LOOP] Event loop started");

    let music = Arc::new(Mutex::new(Music::new()));
    let log_reader = Arc::new(Mutex::new(LogReader::new()));

    connect_to_obs(app).await;

    let mut last_client: &Client = CLIENT_NOT_FOUND;
    let mut last_music_update = Instant::now();

    loop {
        let current_client = get_current_client().unwrap_or(CLIENT_NOT_FOUND);

        if current_client != last_client {
            update_client(current_client, app, &log_reader).await;
            last_client = current_client;
        }

        if last_music_update.elapsed().as_millis() > MUSIC_COOLDOWN_IN_MS {
            last_music_update = Instant::now();

            tokio::spawn({
                let music_clone = music.clone();
                let music_app_handle = app.clone();

                async move {
                    if !read_settings(&music_app_handle).await.music_enable {
                        return;
                    }

                    let mut music = music_clone.lock().await;
                    if let Err(e) = music.update(&music_app_handle).await {
                        error!("[DJ] Error while updating music : {}", e);
                    }
                }
            });
        }

        let log_reader_handle = tokio::spawn({
            let log_reader_clone = log_reader.clone();
            let log_reader_app_handle = app.clone();

            async move {
                let mut log_reader = log_reader_clone.lock().await;
                if let Err(e) = log_reader.read(log_reader_app_handle, last_client.path).await {
                    error!("[LOG] Error while reading logs : {}", e);
                }
            }
        });

        let _ = tokio::join!(log_reader_handle); // Avoid race conditions
        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}

#[tauri::command]
pub async fn start_event_loop(app: AppHandle) {
    let context = {
        let state = app.state::<AppState>();
        let locked_context = state.context.read().await;
        locked_context.clone()
    };

    if context.event_loop_running {
        debug!("[EVENT LOOP] Event loop already running");
        return;
    }

    {
        let state = app.state::<AppState>();
        let mut locked_context = state.context.write().await;
        locked_context.event_loop_running = true;
    }

    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        info!("[MAIN] Event Loop started...");
        // tokio::time::sleep(Duration::from_secs(20)).await;
        get_loop(&app).await;
    });

    // get_loop(&app).await;
}
