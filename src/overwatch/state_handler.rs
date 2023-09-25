use crate::overwatch::client_state::{ClientState};
use crate::overwatch::game_state::{GameState};

struct StateHandler {
    client_state: ClientState,
    game_state: GameState,
}

impl StateHandler {
    pub fn default() -> Self {
        StateHandler {
            client_state: ClientState::default(),
            game_state: GameState::default(),
        }
    }
}
