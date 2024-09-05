use crate::{
    bots,
    commands::{
        alias::AliasType,
        format::{format_command, format_pack, format_server},
        utils::{get_command_by_group, get_command_by_name},
    },
    modules::auto_queuing_scene::controller::set_screen,
    states::{
        config::{read_settings, Alias, Settings},
        context::update_context,
        structs::IngameStatus,
    },
    tailer::{
        events::ChatEvent,
        parser::{parse_packs, parse_server_address},
    },
};

use log::{debug, info};
use std::error::Error;
use tauri::{AppHandle, Manager};

pub async fn process_line<'a, 'b>(line: String, app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let settings = read_settings(app).await;
    let event = match ChatEvent::from_str(&line) {
        Some(event) => event,
        None => return Ok(()),
    };

    debug!("[PARSER] Event found : {:?}", event);

    match event {
        ChatEvent::LobbyBlank1 | ChatEvent::ServerKick => {
            set_screen(app, &settings, !settings.scenes_hide_in_lobby).await;
            update_context("ingame_status", serde_json::json!(IngameStatus::Lobby), app).await;
        }
        ChatEvent::NewGame1 | ChatEvent::PlayerJoin => {
            set_screen(app, &settings, false).await;
            update_context("ingame_status", serde_json::json!(IngameStatus::Queuing), app).await;
        }
        ChatEvent::BedwarsGameStart => {
            set_screen(app, &settings, true).await;
            update_context("ingame_status", serde_json::json!(IngameStatus::InGame.to_string()), app).await;
        }
        ChatEvent::BedwarsGameEnd => {
            update_context("ingame_status", serde_json::json!(IngameStatus::Lobby), app).await;
            if settings.scenes_toggle_after_game_end && settings.scenes_enable {
                let app = app.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(settings.scenes_delay)).await;
                    set_screen(&app, &settings, false).await;
                });
            }
        }
        ChatEvent::ResourcePack => {
            if settings.pack_enable {
                let packs_vec = parse_packs(&line);
                update_packs(app, packs_vec, &settings).await;
            }
        }
        ChatEvent::ServerJoin => {
            if settings.server_enable {
                if let Some(address) = parse_server_address(&line) {
                    update_server(app, address, &settings).await;
                }
            }
        }
    }

    Ok(())
}

async fn update_pack_aliases(app: &AppHandle, packs_vec: &[String]) {
    let mut settings = read_settings(app).await;
    settings.aliases.packs.clear();

    for pack in packs_vec.iter() {
        settings.aliases.packs.insert(pack.to_string(), Alias::default());
    }

    info!("[PARSER] Aliases updated : {:?}", settings.aliases.packs);

    let payload = serde_json::json!({ "aliases": { "packs": settings.aliases.packs } });
    let _ = app.emit_all("update-context", payload);
}

pub async fn update_packs(app: &AppHandle, packs_vec: Vec<String>, settings: &Settings) {
    update_pack_aliases(app, &packs_vec).await;
    update_context("resource_packs_raw", serde_json::json!(packs_vec), app).await;

    let packs = format_pack(app).await;
    info!("[PARSER] Packs updated");

    let bot_response = settings.pack_command_text.replace("{pack}", &packs);
    let _ = bots::update_command(app, "pack", &bot_response).await;

    if settings.pack_announcements_enable {
        let announcement = settings.pack_announcements_text.replace("{pack}", &packs);
        let _ = bots::announce(app, announcement).await;
    }
}

pub async fn update_server(app: &AppHandle, server_address: &str, settings: &Settings) {
    update_context("server_raw", serde_json::json!(server_address), app).await;

    let server_address = format_server(app).await;
    info!("[PARSER] Server updated : {}", server_address);

    let bot_response = settings.server_command_text.replace("{server}", &server_address);
    let _ = bots::update_command(app, "ip", &bot_response).await;

    if settings.server_announcements_enable {
        let announcement = settings.server_announcements_text.replace("{server}", &server_address);
        let _ = bots::announce(app, announcement).await;
    }
}

#[tauri::command]
pub async fn update_command(group: &str, app: AppHandle) -> Result<(), String> {
    let settings = read_settings(&app).await;

    let cmd_name = match AliasType::from_str(group) {
        Ok(alias_enum) => get_command_by_group(alias_enum),
        Err(_) => "music".to_string(),
    };

    let mut command = match AliasType::from_str(group) {
        Ok(alias_enum) => match alias_enum {
            AliasType::Pack => settings.pack_command_text.clone(),
            AliasType::Server => settings.server_command_text.clone(),
        },
        Err(_) => settings.music_command_text.clone(),
    };

    command = format_command(&cmd_name, command, &app).await?;

    match get_command_by_name(&cmd_name) {
        Some(command_name) => bots::update_command(&app, command_name, &command)
            .await
            .map_err(|e| e.to_string())?,
        None => return Err("Command not found, this should never happen".into()),
    };

    Ok(())
}

#[tauri::command]
pub async fn hide_overlay(app: AppHandle) -> Result<(), String> {
    let formatted_packs = format_pack(&app).await;
    let settings = read_settings(&app).await;

    let bot_response = settings.pack_command_text.replace("{pack}", formatted_packs.clone().as_str());
    let _ = bots::update_command(&app, "pack", &bot_response).await;

    Ok(())
}
