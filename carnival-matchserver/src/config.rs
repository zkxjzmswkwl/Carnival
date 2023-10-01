use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    api: String,
    start_game: bool,
    start_ws_server: bool,
    start_discord_bot: bool,
    discord_bot_token: String,
}

impl Config {
    pub fn load() -> Config {
        let toml_str = read_to_string("config.toml").unwrap_or_else(|err| {
            eprintln!("Failed to read config.toml: {err}");
            std::process::exit(1);
        });

        toml::from_str(&toml_str).unwrap_or_else(|err| {
            eprintln!("Failed to parse config.toml: {err}");
            std::process::exit(1);
        })
    }
}
