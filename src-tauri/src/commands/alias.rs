use log::{error, info};
use tauri::State;

use crate::states::config::{save_settings_to_file, Alias, Settings, SettingsState};

pub enum AliasType {
    Pack,
    Server,
}

impl AliasType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "packs" => Ok(Self::Pack),
            "servers" => Ok(Self::Server),
            _ => Err(String::from("Invalid alias type")),
        }
    }
}

pub fn get_alias(group: AliasType, name: &str, settings: &Settings) -> Option<String> {
    let aliases = match group {
        AliasType::Pack => settings.aliases.packs.clone(),
        AliasType::Server => settings.aliases.servers.clone(),
    };

    return aliases.get(name).map(|item| item.alias.clone());
}

#[tauri::command]
pub async fn update_alias(group: String, name: String, alias: String, state: State<'_, SettingsState>) -> Result<(), String> {
    let mut locked_settings = state.settings.write().await;

    {
        let aliases = match AliasType::from_str(&group)? {
            AliasType::Pack => &mut locked_settings.aliases.packs,
            AliasType::Server => &mut locked_settings.aliases.servers,
        };

        match aliases.get_mut(&name) {
            Some(old_alias) => old_alias.alias = alias,
            None => {
                aliases.insert(name, Alias::new(alias, false));
            }
        };
    }

    if let Err(error) = save_settings_to_file(&locked_settings) {
        error!("[ALIAS] Failed to save settings: {}", error);
        return Err(error.to_string());
    }

    info!("[ALIAS] Alias updated successfully, {}", group);

    Ok(())
}

#[tauri::command]
pub async fn delete_alias(group: String, name: String, state: State<'_, SettingsState>) -> Result<(), String> {
    let mut locked_settings = state.settings.write().await;

    let mut aliases = match AliasType::from_str(&group)? {
        AliasType::Pack => locked_settings.aliases.packs.clone(),
        AliasType::Server => locked_settings.aliases.servers.clone(),
    };

    if aliases.remove(&name).is_none() {
        let error_string = format!("Alias {} does not exist", name);
        return Err(error_string);
    }

    match AliasType::from_str(&group)? {
        AliasType::Pack => locked_settings.aliases.packs = aliases,
        AliasType::Server => locked_settings.aliases.servers = aliases,
    }

    if let Err(error) = save_settings_to_file(&locked_settings) {
        error!("[ALIAS] Failed to save settings: {}", error);
        return Err(error.to_string());
    }

    Ok(())
}
