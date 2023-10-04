use std::{sync::mpsc, thread};

use crate::{
    commons::types::ResolvedOverwatchMatch,
    config::Config,
    overwatch::dyn_actions::DynamicActionChain,
};
use overwatch::state_handler::StateHandler;
use tracing_subscriber::filter::LevelFilter;
use color_eyre::eyre::Result;

mod commons;
mod config;
mod connection;
mod input;
mod overwatch;

#[tokio::main]
async fn main() -> Result<()> {
    let mut log_level = LevelFilter::ERROR;
    if cfg!(debug_assertions) {
        log_level = LevelFilter::TRACE;
    }

    color_eyre::install()?;
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(log_level)
        .init();

    overwatch::prelude()?;
    let mut state_handler: StateHandler = StateHandler::default();
    state_handler.restore();
    println!("{state_handler:#?}");

    let config = Config::load();
    println!("{config:#?}");

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        connection::connect(tx);
    });

    let mut action_chains = overwatch::static_actions::ActionChain::default();
    action_chains.load().unwrap();

    loop {
        // recv blocks until the webserver tells us a match is ready.
        if let Ok(recv) = rx.recv() {
            match serde_json::from_str::<ResolvedOverwatchMatch>(&recv) {
                Ok(resolved_match) => {
                    println!("{:#?}", resolved_match);
                    action_chains
                        .invoke_chain("custom_lobby")
                        .invoke_chain("move_self_spec")
                        .invoke_chain("set_preset")
                        .invoke_chain("set_invite_only");

                    resolved_match.resolved_teams.invite();
                },
                Err(e) => panic!("{e}")
            }
        }
    }
}
