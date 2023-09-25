pub struct GameState {
    blue_team: Vec<String>,
    red_team: Vec<String>,
}

impl GameState {
    pub fn default() -> Self {
        GameState {
            blue_team: Vec::new(),
            red_team: Vec::new(),
        }
    }
}
