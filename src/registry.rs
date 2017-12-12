
use std::collections::HashMap;
use std::vec::Vec;
use ws::Sender;

// TODO: Don't allow the same client to be subscribed to the same
// event multiple times.

pub struct ClientRegistry {
    clients: HashMap<String, Vec<Sender>>,
}

impl ClientRegistry {
    pub fn new() -> ClientRegistry {
        ClientRegistry {
            clients: HashMap::new()
        }
    }

    pub fn add(&mut self, events: &[String], client: &Sender) {
        for event in events {
            self.clients.entry(event.to_string()).or_insert_with(Vec::new).push(client.clone());
        }
    }

    pub fn remove(&mut self) {
    }

    pub fn emit(&mut self, event: String, data: String) {
        match self.clients.get(&event) {
            Some(clients) => {
                for client in clients {
                    client.send(data.clone());
                }
            },
            _ => {
                println!("Nothing");
            }
        }
    }
}
