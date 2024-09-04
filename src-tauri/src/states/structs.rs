use crate::api::twitch::FlowEmote;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Streamer {
    pub display_name: String,
    pub id: String,
    pub color: String,
    pub avatar_url: String,
    pub emotes: Vec<FlowEmote>,
}

impl Default for Streamer {
    fn default() -> Self {
        Self {
            display_name: "Streamer".to_string(),
            id: "".to_string(),
            color: "#6441a5".to_string(),
            avatar_url: "".to_string(),
            emotes: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum IngameStatus {
    Lobby,
    Queuing,
    InGame,
    Unknown,
}

impl std::fmt::Display for IngameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngameStatus::Lobby => write!(f, "Lobby"),
            IngameStatus::Queuing => write!(f, "Queuing"),
            IngameStatus::InGame => write!(f, "In Game"),
            IngameStatus::Unknown => write!(f, "?"),
        }
    }
}
