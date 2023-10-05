use std::{sync::mpsc, thread};

use crate::{
    commons::types::ResolvedOverwatchMatch,
    config::Config,
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

    let mut _state_handler: StateHandler = StateHandler::default();
    // state_handler.restore();
    // println!("{state_handler:#?}");

    let config = Config::load();
    println!("{config:#?}");

    let (tx, rx) = mpsc::channel::<String>();
    // Need to clone the sender so we are able to pass the original to the websocket connection thread
    // since Sender/Receivers are not threadsafe. 
    let tx1 = tx.clone();

    thread::spawn(move || {
        log::info!("{:#?}", thread::current().id());
        connection::connect(tx);
    });

    let mut action_chains = overwatch::static_actions::ActionChain::default();
    action_chains.load().unwrap();

    loop {
        // recv blocks until the webserver tells us a match is ready.
        if let Ok(recv) = rx.recv() {
            match serde_json::from_str::<ResolvedOverwatchMatch>(&recv) {
                Ok(resolved_match) => {
                    overwatch::prelude()?;
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
