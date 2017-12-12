
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

    pub fn add(&mut self, events: &[String], client: &Sender) -> bool {
        for event in events {
            // This event already has this client. So we don't want to do anything.
            if self.event_has_client(event.to_string(), client.clone()) {
                return false;
            }
            // New client for this event, subscribe them to it.
            self.clients.entry(event.to_string()).or_insert_with(Vec::new).push(client.clone());
        }
        return true;
    }

    pub fn remove(&mut self) {
    }

    pub fn emit(&mut self, event: String, data: String) {
        if let Some(clients) = self.clients.get(&event) {
            for client in clients {
                let _ = client.send(data.clone());
            }
        }
    }

    fn event_has_client(&mut self, event: String, client: Sender) -> bool {
        if let Some(clients) = self.clients.get(&event) {
            for connected in clients {
                if client.connection_id() == connected.connection_id() {
                    return true;
                }
            }
        }
        return false;
    }
}
