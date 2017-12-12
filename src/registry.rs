
use std::collections::HashMap;
use std::vec::Vec;
use ws::Sender;

pub struct ClientRegistry {
    clients: HashMap<String, Vec<Sender>>,
}

impl ClientRegistry {
    pub fn new() -> ClientRegistry {
        ClientRegistry {
            clients: HashMap::new()
        }
    }

    pub fn add(&mut self, events: &[String], client: Sender) {
        for event in events {
            // TODO: Fix this
            // self.clients.entry(event.to_string()).or_insert_with(Vec::new).push(client);
        }
    }

    pub fn remove(&mut self) {
    }
}
