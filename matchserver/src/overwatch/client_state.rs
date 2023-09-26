use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
enum Menu {
    MainMenu,
    PlayMenu,
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
    pub fn test_set_dummy_data(&mut self) {
        self.client_state = Menu::CustomSettingsRoot;
        self.previous_state = Menu::CustomLobby;
    }
}
