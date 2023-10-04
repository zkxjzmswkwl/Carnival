use crate::input;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::{thread, time};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
enum ActionStyle {
    MouseAction(MouseAction),
    KeyboardAction(KeyboardAction),
}

// this seems bad
// bad as in not good but still performant
// NOTE(Carter): Seems fine to be tbh
impl ActionStyle {
    fn invoke(&self) {
        match &self {
            ActionStyle::KeyboardAction(action) => action.invoke(),
            ActionStyle::MouseAction(action) => action.invoke(),
        }
    }
}
trait Action {
    fn invoke(&self) {}
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash)]
struct MouseAction {
    x: i32,
    y: i32,
    delay: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
struct KeyboardAction {
    input: String,
    delay: u64,
}

impl Action for MouseAction {
    fn invoke(&self) {
        log::debug!(" Invoking MouseAction: \n {self:#?}");
        input::click(self.x, self.y);
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}
impl Action for KeyboardAction {
    fn invoke(&self) {
        log::debug!("Invoking KeyboardAction: \n {self:#?}");
        winput::send_str(&self.input);
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ActionChain(HashMap<String, Vec<ActionStyle>>);

impl ActionChain {
    pub fn invoke_chain(&self, name: &str) -> &Self {
        let chain = self
            .0
            .get(name)
            .unwrap_or_else(|| panic!("No chain by the name of \"{name}\""));
        let chain_len = chain.len();
        log::debug!("Chain \"{name}\" has length of {chain_len}");

        chain.iter().for_each(|action| {
            // see the comment "this seems bad"
            action.invoke()
        });
        self
    }

    // its either use json or parse a custom toml structure.
    // KEEP IT SIMPLE STUPID:  https://www.youtube.com/watch?v=k0qmkQGqpM8
    pub fn load(&mut self) -> Result<&Self, Box<dyn Error>> {
        *self = serde_json::from_str(&read_to_string("action_chains.json")?)?;
        Ok(self)
    }
}
