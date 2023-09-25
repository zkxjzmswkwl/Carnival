use overwatch::{state_handler::StateHandler, actions::Actions};
use crate::config::Config;
use tracing::metadata::LevelFilter;

mod input;
mod overwatch;
mod config;

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


    overwatch::client_prelude();
    let mut state_handler: StateHandler = StateHandler::default();
    state_handler.test_set_dummy_data().dump();
    state_handler.restore();
    log::debug!("{:#?}", state_handler);

    let config = Config::load();
     log::debug!("{:#?}", config);

    let mut action_chains = Actions::default();
    action_chains.load();
    action_chains.invoke_chain("custom_lobby".to_string());
    action_chains.invoke_chain("move_self_spec".to_string());

    Ok(())
}
