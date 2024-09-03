use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait TwitchBot: Send + Sync {
    fn get_name(&self) -> &str;
    async fn initialize(&self) -> Result<(), Box<dyn Error>>;
    async fn update_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>>;
}

// fn get_commands(&self) -> Option<String>;
// fn get_command(&self, command: &str) -> Result<(), Box<dyn Error>>;
// fn create_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>>;
// fn update_command(&self, command: &str, value: &str) -> Result<(), Box<dyn Error>>;
pub struct DefaultBot {}

#[async_trait]
impl TwitchBot for DefaultBot {
    fn get_name(&self) -> &str {
        "none"
    }
    async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn update_command(&self, _command: &str, _value: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
// fn get_command(&self, _command: &str) -> Result<(), Box<dyn Error>> {
//     Ok(())
// }
// fn get_commands(&self) -> Option<String> {
//     Ok("Oui".to_string())
// }
// fn create_command(&self, _command: &str, _value: &str) -> Result<(), Box<dyn Error>> {
//     Ok(())
// }
// fn update_command(&self, _command: &str, _value: &str) -> Result<(), Box<dyn Error>> {
//     Ok(())
// }
