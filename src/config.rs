use serde::{Serialize, Deserialize};
use std::fs::read_to_string;
use toml;


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    api: String,
    start_game: bool,
    start_ws_server: bool,
    start_discord_bot: bool,
    discord_bot_token: String
}

impl Config {
    pub fn load() -> Config {
        let toml_str = read_to_string("config.toml").expect("config.toml: failed to read");
        let toml = toml::from_str(&toml_str);
        match toml {
            Ok(i) => i,
            Err(e) => {
                panic!("{e}");
            }
        }
    }
}
