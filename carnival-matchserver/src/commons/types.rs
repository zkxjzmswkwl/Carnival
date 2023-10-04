use serde::{Serialize, Deserialize};

use crate::overwatch::dyn_actions::DynamicActionChain;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct OverwatchMatch {
    pub id: i32,
    pub map_id: i32,
    pub winner: u8,
    pub status: u8,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ResolvedTeams {
    pub blue: Vec<String>,
    pub red: Vec<String>,
}

impl ResolvedTeams {
    pub fn invite(&self) {
        self.blue.iter().for_each(|btag| {
            DynamicActionChain::generate_invite_chain(btag.to_string(), 1).invoke();
        });
        self.red.iter().for_each(|btag| {
            DynamicActionChain::generate_invite_chain(btag.to_string(), 2).invoke();
        });
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ResolvedOverwatchMatch {
    pub overwatch_match: OverwatchMatch,
    pub resolved_teams: ResolvedTeams,
}