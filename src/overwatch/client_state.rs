#[allow(dead_code)]
#[derive(PartialEq, Debug)]
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
    Unknown
}

#[derive(Debug)]
pub struct ClientState {
    client_state: Menu,
    previous_state: Menu
}

impl ClientState {
    pub fn default() -> Self {
        ClientState {
            client_state: Menu::Unknown,
            previous_state: Menu::Unknown
        }
    }
}


