use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Command {
    name: String,
    message: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    enable: bool,
    command_text: String,
    prefix: String,
    commands: Vec<Command>,
}

impl Config {
    fn save(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(file_path)?;
        serde_json::to_writer(file, self)?;

        Ok(())
    }

    fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path)?;
        let config: Config = serde_json::from_reader(file)?;

        Ok(config)
    }
}
