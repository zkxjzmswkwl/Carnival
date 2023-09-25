use overwatch::{state_handler::StateHandler, actions::Actions};
use crate::config::Config;

mod input;
mod overwatch;
mod config;

fn main() {
    overwatch::client_prelude();
    let mut state_handler: StateHandler = StateHandler::default();
    // state_handler.test_set_dummy_data().dump();
    state_handler.restore();
    println!("{:#?}", state_handler);

    let config = Config::load();
    println!("{:#?}", config);

    let mut action_chains = Actions::default();
    action_chains.load();
    action_chains.invoke_chain("custom_lobby".to_string());
    action_chains.invoke_chain("move_self_spec".to_string());
    action_chains.invoke_chain("set_preset".to_string());
    action_chains.invoke_chain("set_invite_only".to_string());
}
