use std::{sync::mpsc, thread, time};

use crate::{commons::types::ResolvedOverwatchMatch, config::Config};
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
    let mut log_level = LevelFilter::ERROR;
    if cfg!(debug_assertions) {
        log_level = LevelFilter::TRACE;
    }

    color_eyre::install()?;
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(log_level)
        .init();
 
    unsafe {
        let tank: Tank = Tank::new();
        // We can't assume the initial frame rate - so we can't look for `FPS: 60`.
        // The display fluxuates between 60/59 each frame.
        // So we can for the only constant value in the fps counter,
        // then read a few bytes past that to grab current fps.
        let str_test = tank.find_str("FPS: ", 8);

        loop {
            for result in &str_test {
                if let Some(new) = tank.read_str(result.address, 9) {
                    println!("{:X}: {}", result.address, new);
                }
            }
        }
        str_test
            .iter()
            .for_each(|str| println!("{} @ 0x{:X}", str.value, str.address));
        println!("Count pre filter: {}", str_test.len());
        println!("==================================================");
        thread::sleep(time::Duration::from_millis(3000));
        tank.retain_valid(&mut str_test);
        str_test
            .iter()
            .for_each(|str| println!("{} @ 0x{:X}", str.value, str.address));
        println!("Count post filter: {}", str_test.len());
        println!("==================================================");
    }
    
    let mut _state_handler: StateHandler = StateHandler::default();
    let config = Config::load();

    // Setup ipc so the websocket connection thread can pass game objects to the main thread.
    let (tx, rx) = mpsc::channel::<String>();
    // Need to clone the sender so we are able to pass the original to the websocket connection thread
    // since Sender/Receivers are not threadsafe. 
    let tx1 = tx.clone();

    thread::spawn(move || {
        connection::connect(tx, &mut state_handler.game_state);
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
                }
                Err(e) => panic!("{e}"),
            }
        }
    }
}
