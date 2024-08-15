use std::{sync::mpsc, thread};

use crate::{commons::types::ResolvedOverwatchMatch, config::Config, overwatch::map::Map};
use color_eyre::eyre::Result;
use league::league_client::{self, change_status};
use overwatch::{dontlookblizzard::Tank, state_handler::StateHandler};
use tracing_subscriber::filter::LevelFilter;

mod commons;
mod config;
mod connection;
mod input;
mod league;
mod overwatch;

#[tokio::main]
async fn main() -> Result<()> {
    let mut lcu = shaco::rest::RESTClient::new().unwrap();
    change_status(&mut lcu).await;
    Ok(())
    // let mut log_level = LevelFilter::INFO;
    // if cfg!(debug_assertions) {
    //     log_level = LevelFilter::TRACE;
    // }

    // color_eyre::install()?;
    // tracing_subscriber::fmt()
    //     .pretty()
    //     .with_max_level(log_level)
    //     .init();

    // let mut state_handler: StateHandler = StateHandler::default();
    // let mut tank: Tank = Tank::new();
    // let mut action_chains = overwatch::static_actions::ActionChain::default();
    // action_chains.load().unwrap();
    // overwatch::prelude()?;
    // // TODO: Move this to its own thread.
    // unsafe {
    //     state_handler.client_state.run_initial_scans(&mut tank);
    //     loop {
    //         let client_state = state_handler.client_state.determine(&tank);
    //         client_state.advance(&action_chains, &mut state_handler.game_state);
    //         log::info!("{:#?}", client_state);
    //     }
    // }
    let mut _state_handler: StateHandler = StateHandler::default();
    // state_handler.restore();
    // println!("{state_handler:#?}");

    // let _config = Config::load();
    let (tx, rx) = mpsc::channel::<String>();
    // Need to clone the sender so we are able to pass the original to the websocket connection thread
    // since Sender/Receivers are not threadsafe. 
    let tx1 = tx.clone();

    thread::spawn(move || {
        log::info!("{:#?}", thread::current().id());
        connection::connect(tx);
    });

    // log::info!(
    //     "Connection thread id {:#?}",
    //     connection_thread.thread().id()
    // );

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

    //                 let map: Map =
    //                     unsafe { std::mem::transmute(resolved_match.overwatch_match.map_id) };
    //                 resolved_match.resolved_teams.invite();
    //             }
    //             Err(e) => panic!("{e}"),
    //         }
    //     }
    // }
}
