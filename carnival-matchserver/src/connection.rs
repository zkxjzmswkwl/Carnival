use std::{sync::mpsc, thread, time::Duration};

use tungstenite::Message;
use url::Url;

use crate::overwatch::game_state::GameState;

pub fn connect(ipc: mpsc::Sender<String>) {
    let (mut socket, _resp) = tungstenite::connect(Url::parse("wss://carnival.ryswick.net/ws/notifications").unwrap())
        .expect("No connection made.");

    // Authenticate with proper matchserver token (generated)
    socket.send(Message::Text("auth:debugtoken69".into())).unwrap();
    if let Ok(auth_resp) = socket.read() {
        log::info!("{}", auth_resp.to_string());

        loop {
            if game_state.has_game {
                log::debug!("GameState's has_game is true, not asking the webserver for another match.");
                thread::sleep(Duration::from_secs(5));
                continue;
            }
            // Send a ping every few seconds.
            socket.send(Message::Text("match?".into())).unwrap();

            if let Ok(resp) = socket.read() {
                log::info!("{}", resp);
                let resp_str = resp.to_string();
                // If the message received back from the server does not return "match"
                if resp_str != "match" {
                    // Check again in 5 seconds.
                    // let _ = socket.send(Message::Text("ack".to_string()));
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }

                // If it does, listen for another packet containing the match data.
                if let Ok(match_resp) = socket.read() {
                    // Tell the webserver that we have the data 
                    let _ = socket.send(Message::Text("match ack".to_string()));
                    match ipc.send(match_resp.to_string()) {
                        Ok(_) => {
                            // TODO: Need to recv ack from server for this.
                            socket.send(Message::Text("match lobby".to_string()));
                        },
                        Err(e) => eprintln!("{e}")
                    }
                }
            }
        }
    }
}
