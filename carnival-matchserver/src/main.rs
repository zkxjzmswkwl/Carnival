use std::{sync::mpsc::{self, Sender}, thread, time::{self, Instant}, arch::asm};

use crate::{commons::types::ResolvedOverwatchMatch, config::Config, overwatch::dontlookblizzard::{ScanResult, THREADSAFE_MEMORY_BASIC_INFO}};
use color_eyre::eyre::Result;
use overwatch::{dontlookblizzard::Tank, state_handler::StateHandler};
use tracing_subscriber::filter::LevelFilter;
use windows::Win32::System::Memory::{MEMORY_BASIC_INFORMATION, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE, PAGE_TYPE};
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
    // TODO: Move this to its own thread.
    unsafe {
        state_handler.client_state.run_initial_scans(&mut tank);
        loop {
            let client_state = state_handler.client_state.determine(&tank);
            log::info!("{:#?}", client_state);
            thread::sleep(time::Duration::from_millis(500));
        }
    }
    
    // None of this shit will ever fire until ^ todo is done.
    let config = Config::load();

    // Setup ipc so the websocket connection thread can pass game objects to the main thread.
    let (tx, rx) = mpsc::channel::<String>();
    let connection_thread = thread::spawn(move || {
        connection::connect(tx);
    });

    log::info!("Connection thread id {:#?}", connection_thread.thread().id());


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
                }
                Err(e) => panic!("{e}"),
            }
        }
    }
}
