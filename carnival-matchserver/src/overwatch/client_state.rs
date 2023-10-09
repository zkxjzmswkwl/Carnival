use serde::{Deserialize, Serialize};

use crate::overwatch::dontlookblizzard::CachedScan;

use super::dontlookblizzard::Tank;

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
            return Menu::InGame;
        }

        // Current engine build #
        if overwatch.turboscan("2.6.1.1 - 116944").len() > 8 {
            return Menu::MainMenu;
        }

        if overwatch.turboscan("Jump into").len() > 2 {
            return Menu::PlayMenu;
        }

        // This is inconsistent. I've seen the value as low as 18 and as high as 52.
        if overwatch.turboscan("tinder watch").len() > 48 {
            return Menu::CustomList;
        }

        if overwatch.turboscan("Add AI").len() > 2 {
            return Menu::CustomLobby;
        }

        return Menu::Unknown;
    }

    pub fn test_set_dummy_data(&mut self) {
        self.client_state = Menu::CustomSettingsRoot;
        self.previous_state = Menu::CustomLobby;
    }
}
