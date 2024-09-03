pub struct Stats {
    pub username: String,
    pub uuid: String,
    pub labels_folder: String,
    pub current_stats: CurrentStats,
}

struct CurrentStats {
    final_kills: u32,
    wins: u32,
    level: f32,
}

fn get_uuid(username: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);
    let resp = reqwest::blocking::get(&url)?.text()?;
    let json: Value = serde_json::from_str(&resp)?;
    let uuid = json["id"].to_string();
    Ok(uuid)
}

// fn start_session() -> {
//     // send request to api
// }

impl Stats {
    fn new(username: &str, labels_folder: &str) -> Result<Self, Box<dyn Error>> {
        let uuid = get_uuid(username)?;
        Ok(Self {
            username: username.to_string(),
            uuid,
            labels_folder: labels_folder.to_string(),
            current_stats: CurrentStats {
                final_kills: 0,
                wins: 0,
                level: 0.0,
            },
        })
    }
}
