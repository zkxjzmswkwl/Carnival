use std::{thread, sync::mpsc};

use crate::{commons::types::ResolvedOverwatchMatch, config::Config, overwatch::map::Map};
use color_eyre::eyre::Result;
use overwatch::{dontlookblizzard::Tank, state_handler::StateHandler};
use tracing_subscriber::filter::LevelFilter;
mod commons;
mod config;
mod connection;
mod input;
mod overwatch;

#[tokio::main]
async fn main() -> Result<()> {
    let mut log_level = LevelFilter::INFO;
    if cfg!(debug_assertions) {
        log_level = LevelFilter::TRACE;
    }

    color_eyre::install()?;
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(log_level)
        .init();

    let mut state_handler: StateHandler = StateHandler::default();
    let mut tank: Tank = Tank::new();
    let mut action_chains = overwatch::static_actions::ActionChain::default();
    action_chains.load().unwrap();
    overwatch::prelude()?;
    // TODO: Move this to its own thread.
    unsafe {
        state_handler.client_state.run_initial_scans(&mut tank);
        loop {
            let client_state = state_handler.client_state.determine(&tank);
            client_state.advance(&action_chains, &mut state_handler.game_state);
            log::info!("{:#?}", client_state);
        }
    }
    
    let _config = Config::load();

    // Setup ipc so the websocket connection thread can pass game objects to the main thread.
    let (tx, rx) = mpsc::channel::<String>();

    let connection_thread = thread::spawn(move || {
        connection::connect(tx, &mut state_handler.game_state);
    });

    log::info!("Connection thread id {:#?}", connection_thread.thread().id());

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

                    let map: Map = unsafe { std::mem::transmute(resolved_match.overwatch_match.map_id) };
                    resolved_match.resolved_teams.invite();
                }
                Err(e) => panic!("{e}"),
            }
        }
    }
}