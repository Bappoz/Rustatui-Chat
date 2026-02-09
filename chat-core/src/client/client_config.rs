use clap::{Parser};

#[derive(Parser, Debug, Clone)]
#[command(name = "Rusty Chat Client")]
#[command(about = "Connect to Rusty Chat Server", long_about = None)]
pub struct ClientConfig {
    // Username
    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long, default_value = "general")]
    pub room: String,

    #[arg(short, long)]
    pub password: String,

    #[arg(short, long, default_value = "white")]
    pub color: String,

    #[arg(long, default_value = "localhost:4556")]
    pub server: String,
}

impl ClientConfig {
    pub fn parse_agrs() -> Self {
        ClientConfig::parse()
    }

    pub fn get_display_name(&self, anonymous_id: u32) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("Anonymous#{}", anonymous_id)
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if let Some(name) = &self.name {
            if name.len() < 2 || name.len() > 20 {
                return Err("Name must be between 2 and 20 characters.".to_string());
            }
        }

        let valid_color = ["red", "green", "blue", "yellow", "magenta", "cyan", "white"];
        if !valid_color.contains(&self.color.as_str()) {
            return Err(format!("Invalid color. Use {:?}", valid_color));
        }

        Ok(())
    }
}