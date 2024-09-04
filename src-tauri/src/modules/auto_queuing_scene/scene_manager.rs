use log::{debug, error, info};
use obws::{responses::scenes::Scenes, Client};
use std::{sync::Arc, time::Duration};
use tauri::{AppHandle, Manager, State};
use tokio::{sync::RwLock, time};

use crate::states::{
    config::{read_settings, Settings},
    context::update_context,
};

use super::socket_config::SocketConfig;

pub struct SceneManager {
    pub client: Option<Client>,
    pub current_scene: String,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            client: None,
            current_scene: String::new(),
        }
    }

    pub async fn connect(&mut self, settings: &Settings) -> Result<(), String> {
        let socket_config = match settings.scenes_auto_obs_config {
            true => SocketConfig::load(&settings.scenes_obs_config_path).map_err(|e| e.to_string())?,
            false => SocketConfig {
                enabled: true,
                port: settings.scenes_obs_ws_port,
                auth_required: !settings.scenes_obs_ws_password.is_empty(),
                password: settings.scenes_obs_ws_password.clone(),
            },
        };

        info!("[OBS] Attempting to connect...");

        let connection_result = time::timeout(
            Duration::from_secs(5),
            Client::connect("localhost", socket_config.port, Some(socket_config.password)),
        )
        .await;

        match connection_result {
            Ok(Ok(client)) => {
                self.client = Some(client);
                info!("[OBS] Online");
                Ok(())
            }
            Ok(Err(e)) => {
                error!("[OBS] Error while connecting to OBS : {}", e);
                Err(e.to_string())
            } // If the connection fails but within the timeout
            Err(_timeout) => Err("Connection attempt timed out".into()), // If the operation times out
        }
    }

    pub async fn get_scenes_list(&mut self, app: &AppHandle) -> Result<Scenes, String> {
        let settings = read_settings(app).await;

        match &self.client {
            Some(client) => client.scenes().list().await.map_err(|e| e.to_string()),
            None => {
                self.connect(&settings).await?;
                Err("Missing client".to_string())
            }
        }
    }

    pub async fn hide_screen(&mut self, scene: &str) -> Result<(), obws::Error> {
        info!("[OBS] Hiding screen");

        if let Some(client) = &self.client {
            let current_scene = client.scenes().current_program_scene().await?;
            debug!("[OBS] Current scene : {:?}", current_scene);
            debug!("[OBS] Scene : {:?}", scene);

            if current_scene != scene {
                info!("[OBS] Current scene is not the same as the one we want to hide");
                self.current_scene = current_scene;
            }

            client.scenes().set_current_program_scene(scene).await?;
            info!("[OBS] Hide | Current scene : {:?}", self.current_scene);
        }
        Ok(())
    }

    pub async fn show_screen(&self) -> Result<(), String> {
        info!("[OBS] Showing screen");
        info!("[OBS] Show | Current scene : {:?}", self.current_scene);

        if self.current_scene.is_empty() {
            return Ok(()); // No scene to show
        }

        match &self.client {
            Some(client) => client
                .scenes()
                .set_current_program_scene(&self.current_scene)
                .await
                .map_err(|e| e.to_string()),
            None => Err("No client".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct SceneState {
    pub scene_manager: Arc<RwLock<SceneManager>>,
}

impl SceneState {
    pub fn new() -> Self {
        Self {
            scene_manager: Arc::new(RwLock::new(SceneManager::new())),
        }
    }
}

#[tauri::command]
pub async fn get_scenes_list(state: State<'_, SceneState>, app: AppHandle) -> Result<Scenes, String> {
    let mut scene_manager = state.scene_manager.write().await;
    let result = scene_manager.get_scenes_list(&app).await;

    if let Err(e) = &result {
        error!("[OBS] Error while getting scenes list : {}", e);
        update_context("obs_status", serde_json::json!("Offline"), &app).await;
        return Err(e.to_string());
    }

    result
}

#[tauri::command]
pub async fn connect_to_obs(scene_state: State<'_, SceneState>, app: AppHandle) -> Result<(), String> {
    let mut scene_manager = scene_state.scene_manager.write().await;

    if scene_manager.client.is_some() {
        return Ok(());
    }

    let settings = read_settings(&app).await;

    if let Err(e) = scene_manager.connect(&settings).await {
        error!("[OBS] Error while connecting to OBS : {}", e);
        update_context("obs_status", serde_json::json!("Offline"), &app).await;
        return Err(e.to_string());
    }

    update_context("obs_status", serde_json::json!("Online"), &app).await;

    Ok(())
}

#[tauri::command]
pub async fn disconnect_from_obs(app: AppHandle) -> Result<(), String> {
    let state = app.state::<SceneState>().clone();
    let mut scene_manager = state.scene_manager.write().await;
    scene_manager.client = None;

    info!("[OBS] Disconnected from OBS");

    update_context("obs_status", serde_json::json!("Offline"), &app).await;

    Ok(())
}
