use crate::overwatch::client_state::ClientState;
use crate::overwatch::game_state::GameState;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write, File};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StateHandler {
    client_state: ClientState,
    pub game_state: GameState,
}

impl StateHandler {
    #[allow(dead_code)]
    pub fn test_set_dummy_data(&mut self) -> &mut Self {
        self.client_state.test_set_dummy_data();
        self.game_state.test_set_dummy_data();
        self
    }

    #[allow(dead_code)]
    fn to_toml(&self) -> Option<String> {
        if let Ok(json) = toml::to_string(self) {
            Some(json)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        if let Some(json_str) = self.to_toml() {
            if File::create("state_handler.toml").is_err() {
                panic!("nope")
            }
            write("state_handler.toml", json_str).expect("Couldn't write state_handler.toml");
        }
    }

    pub fn restore(&mut self) {
        let json =
            read_to_string("state_handler.toml").expect("state_handler.json: failed to read");
        if let Ok(serialized_dump) = toml::from_str(&json) {
            *self = serialized_dump;
        }
    }
}
