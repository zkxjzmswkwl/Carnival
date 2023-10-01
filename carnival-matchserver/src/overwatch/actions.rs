use crate::input;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Duration;
use std::{thread, time};

#[allow(dead_code)]

trait Action {
    fn invoke() {
    }
}


enum ActionStyle {
    MouseAction(MouseAction),
    KeyboardAction(KeyboardAction),
}
struct MouseAction {
    x: i32,
    y: i32, 
    delay: u64,
}
struct KeyboardAction {
    keycode: String,
    delay: u64,
}

struct ActionChain(HashMap<String, Vec<ActionType>>);

impl ActionChain {
    pub fn invoke_chain(&self, name: &str) -> &Self {
        let chain = self.chains.get(name)
        .unwrap_or_else(|| panic!("No chain by the name of \"{name}\""));
        let chain_len = chain.len();
        log::debug!("Chain \"{name}\" has length of {chain_len}");

        chain.iter().for_each(|action| {
            //action.invoke();
        });
        self
    }

    fn to_json(&self) { 
        let json = serde_json::to_str(&self.0);
    }
    fn from_json(&mut self, action_chains: &str) -> Result<&Self, Box<dyn Error>> {
        let json = serde_json::from_str(action_chains)?;
        *self = json;
        Ok(self)
    }
}





#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash)] // TODO: Figure out why I need Copy, Clone and Hash? cbf
struct Action {
    x: i32,
    y: i32,
    delay: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Actions {
    chains: HashMap<String, Vec<Action>>,
}

impl Action {
    pub fn invoke(&self) {
        log::debug!(" Invoking action: \n {self:#?}");
        input::click(self.x, self.y);
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}

impl Actions {
    pub fn invoke_chain(&self, name: &str) -> &Self {
        let chain = self.chains.get(name)
        .unwrap_or_else(|| panic!("No chain by the name of \"{name}\""));
        let chain_len = chain.len();
        log::debug!("Chain \"{name}\" has length of {chain_len}");

        chain.iter().for_each(|action| {
            action.invoke();
        });
        self
    }

    pub fn load(&mut self) {
        let toml_str =
            read_to_string("action_chains.toml").expect("action_chains.toml: failed to read");
        let toml = toml::from_str(&toml_str);
        match toml {
            Ok(i) => *self = i,
            Err(e) => {
                panic!("{e}");
            }
        }
    }
}
