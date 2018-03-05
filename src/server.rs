use ws;
use serde_json;

use packet;
use response_builder::Response;
use registry::ClientRegistry;

use std::sync::Mutex;

trait PacketHandler {
    fn handle_message(&mut self, msg: &ws::Message) -> Response;
}

struct Server<'a> {
    ws: ws::Sender,
    registry: &'a Mutex<ClientRegistry>
}

impl<'a> PacketHandler for Server<'a> {
    fn handle_message(&mut self, msg: &ws::Message) -> Response {
        let parsed: serde_json::Result<packet::Context> =
                serde_json::from_str(&msg.to_string());

        if parsed.is_err() {
            return Response::Error(String::from("Invalid context."));
        }

        let deserialized = parsed.unwrap();
        println!("{:?}", deserialized);

        return Response::Success(serde_json::to_value(deserialized).unwrap());
    }
}

impl<'a> ws::Handler for Server<'a> {        
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Received: {}", msg);
        let _ = self.handle_message(&msg);
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
    }
}

pub fn run_server(host: &str, port: i16) {
    let mut registry = Mutex::new(ClientRegistry::new());
    ws::listen(format!("{}:{}", host, port), |out| Server {
        ws: out,
        registry: &registry
    }).unwrap();
}
