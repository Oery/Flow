#[derive(Debug, PartialEq)]
pub enum ChatEvent {
    ServerKick,
    LobbyBlank1,
    NewGame1,
    BedwarsGameStart,
    BedwarsGameEnd,
    PlayerJoin,
    ResourcePack,
    ServerJoin,
}

impl ChatEvent {
    pub fn from_str(name: &str) -> Option<Self> {
        if name.contains(" [Client thread/INFO]: [OptiFine] Resource packs: ")
            || name.contains(" [Render thread/INFO]: Reloading ResourceManager: ")
        {
            Some(ChatEvent::ResourcePack)
        } else if name.contains(" [Client thread/INFO]: Connecting to ")
            || name.contains(" [Render thread/INFO]: Connecting to ")
            || name.contains(" [Client thread/INFO]: Worker done, connecting to ")
        {
            Some(ChatEvent::ServerJoin)
        } else if name
            .contains(" [Client thread/INFO]: [CHAT] A kick occurred in your connection, so you were put in the Bed Wars lobby!")
        {
            Some(ChatEvent::ServerKick)
        } else if name.contains(" [Client thread/INFO]: [CHAT]                                      ") {
            Some(ChatEvent::LobbyBlank1)
        } else if name.contains(" [Client thread/INFO]: [CHAT] Sending you to mini") {
            Some(ChatEvent::NewGame1)
        } else if name.contains(" [Client thread/INFO]: [CHAT]      Protect your bed and destroy the enemy beds.") {
            Some(ChatEvent::BedwarsGameStart)
        } else if name.contains(" has joined (") {
            Some(ChatEvent::PlayerJoin)
        } else if name.contains("1st Killer - ") {
            Some(ChatEvent::BedwarsGameEnd)
        } else {
            None
        }
    }
}
