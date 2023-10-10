use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub has_game: bool,
    pub game_configured: bool,
    blue_team: Vec<String>,
    red_team: Vec<String>,
}

impl GameState {
    pub fn test_set_dummy_data(&mut self) {
        self.blue_team.push("TestBlue1#123".to_string());
        self.blue_team.push("TestBlue2#123".to_string());
        self.blue_team.push("TestBlue3#123".to_string());
        self.red_team.push("TestRed1#123".to_string());
        self.red_team.push("TestRed2#123".to_string());
        self.red_team.push("TestRed3#123".to_string());
    }
}
