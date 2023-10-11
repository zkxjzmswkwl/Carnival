use core::time;
use std::thread;

use serde::{Deserialize, Serialize};

use crate::overwatch::dontlookblizzard::CachedScan;

use super::{dontlookblizzard::{Tank, ProcessMemory}, static_actions::ActionChain, game_state::GameState};

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub enum Menu {
    MainMenu,
    PlayMenu,
    InGame,
    CustomList,
    CustomLobby,
    CustomSettingsRoot,
    CustomSettingsPreset,
    CustomSettingsLobby,
    CustomSettingsModes,
    CustomSettingsMaps,
    CustomSettingsHeroes,
    CustomSettingsWorkshop,
    #[default]
    Unknown,
}

impl Menu {
    pub fn advance(&self, action_chains: &ActionChain, game_state: &mut GameState) {
        match self {
            Menu::MainMenu    => {
                action_chains.invoke_chain("custom_lobby");
            },
            Menu::CustomLobby => {
                if !game_state.configured  {
                    action_chains
                        .invoke_chain("move_self_spec")
                        .invoke_chain("set_preset")
                        .invoke_chain("set_invite_only");
                    // Don't love this.
                    game_state.configured = true;
                }
            },
            _ => {},
        };
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ClientState {
    client_state: Menu,
    previous_state: Menu,
}

impl ClientState {
    pub unsafe fn run_initial_scans(&mut self, overwatch: &mut Tank) {
        // FPS counter - You don't need to have the counter display enabled for this to work.
        // We can't assume the initial frame rate - so we can't look for `FPS: 60`.
        // The display fluxuates between 60/59 each frame.
        // So we can for the only constant value in the fps counter,
        // then read a few bytes past that to grab current fps.
        let fps_counter_scan = overwatch.find_str("FPS: ", 10);
        log::info!("fps_counter: result len {} | Highest: 0x{:X}", fps_counter_scan.len(), fps_counter_scan.last().unwrap().address);
        overwatch.cached_scans.insert("fps_counter".to_string(), CachedScan::new(fps_counter_scan));
    }

    pub fn determine(&mut self, overwatch: &Tank) -> Menu {
        // Check if in game
        if overwatch.filter_fps_gt_60() > 4 {
            thread::sleep(time::Duration::from_secs(1));
            return Menu::InGame;
        }

        let mut process_memory = unsafe { ProcessMemory::new(overwatch) };
        // Filter pages, if not already filtered, to avoid the nono ones (see PAGE_PROTECTION_MASK and PAGE_TYPE_MASK)
        overwatch.filter_pages(&mut process_memory.pages);

        if overwatch.turboscan(&mut process_memory, "Add AI", 3).len() > 2 {
            return Menu::CustomLobby;
        }

        // Current engine build #
        if overwatch.turboscan(&mut process_memory, "2.7.0.0 - 117535", 9).len() > 8 {
            return Menu::MainMenu;
        }

        if overwatch.turboscan(&mut process_memory, "Jump into", 4).len() > 4 {
            return Menu::PlayMenu;
        }

        // This is inconsistent. I've seen the value as low as 18 and as high as 52.
        if overwatch.turboscan(&mut process_memory, "All Games", 4).len() >= 4 {
            return Menu::CustomList;
        }

        return Menu::Unknown;
    }

    pub fn test_set_dummy_data(&mut self) {
        self.client_state = Menu::CustomSettingsRoot;
        self.previous_state = Menu::CustomLobby;
    }
}
