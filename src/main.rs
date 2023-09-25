use overwatch::state_handler::StateHandler;
use crate::config::{Config};

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
}
