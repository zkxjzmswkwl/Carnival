use std::fs::{File, write, read_to_string};
use serde::{Deserialize, Serialize};
use crate::overwatch::client_state::ClientState;
use crate::overwatch::game_state::GameState;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StateHandler {
    client_state: ClientState,
    game_state: GameState,
}

impl StateHandler {
    pub fn test_set_dummy_data(&mut self) -> &mut Self {
        self.client_state.test_set_dummy_data();
        self.game_state.test_set_dummy_data();
        self
    }

    fn to_json(&self) -> Option<String> {
        if let Ok(json) = serde_json::to_string(self) {
            Some(json)
        } else {
            None
        }
    }

    pub fn dump(&self) {
        if let Some(json_str) = self.to_json() {
            if !File::create("state_handler.json").is_ok() {
                panic!("nope")
            }
            write("state_handler.json", json_str).expect("Couldn't write state_handler.json");
        }
    }

    pub fn restore(&mut self) {
        let json = read_to_string("state_handler.json").expect("state_handler.json: failed to read");
        if let Ok(serialized_dump) = serde_json::from_str(&json) {
            *self = serialized_dump;
        }
    }
}
