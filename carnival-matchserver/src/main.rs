use crate::config::Config;
use overwatch::state_handler::StateHandler;
use tracing_subscriber::filter::LevelFilter;

mod config;
mod input;
mod overwatch;

use color_eyre::eyre::Result;
fn main() -> Result<()> {
    let mut log_level = LevelFilter::ERROR;
    if cfg!(debug_assertions) {
        log_level = LevelFilter::TRACE;
    }

    color_eyre::install()?;
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(log_level)
        .init();

    overwatch::client_prelude()?;
    let mut state_handler: StateHandler = StateHandler::default();
    // state_handler.test_set_dummy_data().dump();
    state_handler.restore();
    println!("{state_handler:#?}");

    let config = Config::load();
    println!("{config:#?}");

    let mut action_chains = overwatch::actions::ActionChain::default();
    action_chains.load().unwrap();
    action_chains
        .invoke_chain("custom_lobby")
        .invoke_chain("move_self_spec")
        .invoke_chain("set_preset")
        .invoke_chain("set_invite_only");

    Ok(())
}
