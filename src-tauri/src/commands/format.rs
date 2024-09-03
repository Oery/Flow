use tauri::AppHandle;

use crate::states::{
    config::read_settings,
    context::{read_context, update_context},
};

use super::{
    alias::{get_alias, AliasType},
    utils::get_default_command,
};

pub async fn format_pack(app: &AppHandle) -> String {
    let mut packs_with_aliases = Vec::new();
    let app_state = read_context(app).await;
    let settings = read_settings(app).await;
    let mut raw_packs = app_state.resource_packs_raw.clone();

    if settings.pack_hide_overlay {
        raw_packs.truncate(1);
    }

    for pack in raw_packs.iter() {
        let pack_string = match get_alias(AliasType::Pack, pack.as_str(), &settings) {
            Some(alias) => alias,
            None => pack.to_string(),
        };
        packs_with_aliases.push(pack_string);
    }

    let formatted_packs = packs_with_aliases.join(", ");
    update_context("resource_pack_str", serde_json::json!(formatted_packs), app).await;

    formatted_packs
}

pub async fn format_server(app: &AppHandle) -> String {
    let app_state = read_context(app).await;
    let settings = read_settings(app).await;

    let formatted_server = match get_alias(AliasType::Server, &app_state.server_raw, &settings) {
        Some(alias) => alias,
        None => app_state.server_raw,
    };

    update_context("server_address", serde_json::json!(formatted_server), app).await;

    formatted_server
}

pub async fn format_music(app: &AppHandle, command: &str) -> String {
    let app_state = read_context(app).await;

    command
        .replace("{music}", &app_state.song_title)
        .replace("{artist}", &app_state.song_author)
}

pub async fn format_command(name: &str, mut command: String, app: &AppHandle) -> Result<String, String> {
    if command.is_empty() {
        command = get_default_command(name)?;
    }

    if command.contains("{pack}") {
        let formatted_packs = format_pack(app).await;
        command = command.replace("{pack}", &formatted_packs);
    }

    if command.contains("{server}") {
        let server = format_server(app).await;
        command = command.replace("{server}", &server)
    }

    if command.contains("{music}") {
        command = format_music(app, &command).await;
    }

    Ok(command)
}
