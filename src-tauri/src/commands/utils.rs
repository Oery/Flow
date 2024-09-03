use crate::states::config::Settings;

use super::alias::AliasType;

pub fn get_default_command(cmd_name: &str) -> Result<String, String> {
    let default = Settings::default();
    match cmd_name {
        "pack_command" => Ok(default.pack_command.clone()),
        "pack_command_text" => Ok(default.pack_command_text.clone()),
        "pack_announcements_text" => Ok(default.pack_announcements_text.clone()),

        "server_command" => Ok(default.server_command.clone()),
        "server_command_text" => Ok(default.server_command_text.clone()),
        "server_announcements_text" => Ok(default.server_announcements_text.clone()),

        "music_command" => Ok(default.music_command.clone()),
        "music_command_text" => Ok(default.music_command_text.clone()),
        "music_announce_text" => Ok(default.music_announce_text.clone()),

        _ => Err("Command not found".to_string()),
    }
}

// TODO: This won't work with custom commands
pub fn get_command_by_name(cmd_name: &str) -> Option<&str> {
    match cmd_name {
        "pack_command_text" => Some("pack"),
        "server_command_text" => Some("ip"),
        "music_command_text" => Some("music"),
        _ => None,
    }
}

pub fn get_command_by_group(group: AliasType) -> String {
    match group {
        AliasType::Pack => "pack_command_text".to_string(),
        AliasType::Server => "server_command_text".to_string(),
    }
}
