use ws;
use serde_json;

use packet;
use response_builder::Response;
use registry::ClientRegistry;

use std::sync::Mutex;

trait PacketHandler {
    fn handle_message(&mut self, msg: &ws::Message) -> Response;
}

struct Server<'registry> {
    ws: ws::Sender,
    registry: &'registry Mutex<ClientRegistry>
}

impl<'registry> PacketHandler for Server<'registry> {
    fn handle_message(&mut self, msg: &ws::Message) -> Response {
        let parsed: serde_json::Result<packet::Context> =
                serde_json::from_str(&msg.to_string());

        if parsed.is_err() {
            return Response::Error(String::from("Invalid context."));
        }

        let deserialized = parsed.unwrap();
        println!("{:?}", deserialized);
        // Send this packet to all the clients that are currently connected.
        // TODO: This should only send it to the `target` plugin that is supposed
        // to be getting this plugin message. However, for now this is being sent
        // to all clients currently for the ease of development and getting this
        // ready for testing.
        //
        self.registry.lock().unwrap().broadcast(None, msg.to_string());

        return Response::Success(serde_json::to_value(deserialized).unwrap());
    }
}

impl<'registry> ws::Handler for Server<'registry> {        
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Received: {}", msg);
        let _ = self.registry.lock().unwrap().add(vec!(String::from("a")), &self.ws);

        let _ = self.handle_message(&msg);
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        println!("Client disconnected.");
    }
}

pub fn run_server(host: &str, port: i16) {
    let mut registry = Mutex::new(ClientRegistry::new());
    ws::listen(format!("{}:{}", host, port), |out| Server {
        ws: out,
        registry: &registry
    }).unwrap();
}
