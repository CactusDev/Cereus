
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

    fn collect_clients(&self, event: Option<String>) -> Vec<Sender> {
        match event {
            Some(e) => {
                if let Some(clients) = self.clients.get(&e) {
                    return clients.clone();
                }
                return vec![];
            },
            None => {
                let mut collected = vec![];
                for key in self.clients.keys() {
                    // Note: This makes the compiler say that it doesn't need to be mutable,
                    // but if you remove it all the things start to error. Never remove the
                    // `mut` here.
                    let mut c = self.clients.get(key);
                    let clients = c.as_mut().unwrap();
                    collected.extend(clients.iter().cloned());
                }
                return collected;
            }
        }
    }

    fn broadcast(&mut self, event: Option<String>, message: String) {
        // Collect the clients that match the criteria
        let clients: Vec<Sender> = self.collect_clients(event);
        // Send the message to all the clients
        for client in clients {
             let _ = client.send(message.to_string());
        }
    }
}
