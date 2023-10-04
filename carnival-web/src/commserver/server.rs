use std::{collections::HashMap, future::Future, pin::Pin};

use simple_websockets::{Event, EventHub, Responder};

use crate::db::services::overwatch_match::ResolvedOverwatchMatch;

#[derive(Debug)]
pub struct MatchServerConn {
    match_server_addr: &'static str,
    hub: EventHub,
    clients: HashMap<u64, Responder>,
    pub outbound_queue: Vec<String>,
}

impl MatchServerConn {
    pub fn new(port: u16) -> Self {
        let ret = Self {
            match_server_addr: "",
            hub: simple_websockets::launch(port).expect("Cannot listen on port {port}"),
            clients: HashMap::new(),
            outbound_queue: Vec::new(),
        };

        ret
    }

    // pub fn listen<F: Fn(&Vec<String>), Fut>(&mut self, receiver_action: F)
    pub async fn listen<F>(&mut self, f: impl Fn() -> Option<ResolvedOverwatchMatch>) {
        loop {
            match self.hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    println!("New client {client_id}");
                    self.clients.insert(client_id, responder);
                }
                Event::Disconnect(client_id) => {
                    println!("Client {client_id} disconnected");
                    self.clients.remove(&client_id);
                }
                Event::Message(client_id, message) => {
                    println!("{client_id}: {:#?}", message);
                    if let Some(responder) = self.clients.get(&client_id) {
                        responder.send(message);
                    }
                }
            }
        }
    }
}
