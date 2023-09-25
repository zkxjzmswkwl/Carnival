use crate::input;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::{thread, time};

#[allow(dead_code)]
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
    pub fn invoke(self) {
        log::debug!(" Invoking action: \n {:#?}", self);

        input::click(self.x, self.y);
        thread::sleep(time::Duration::from_millis(self.delay));
    }
}

impl Actions {
    pub fn invoke_chain(&self, name: String) -> &Self {
        let chain = self
            .chains
            .get(&name)
            .expect(&format!("No chain by the name of \"{}\"", &name));
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
