use ws;
use serde_json;

use packet;
use response_builder::Response;
use registry::{ClientRegistry};

trait PacketHandler {
    fn handle_message(&mut self, msg: &ws::Message) -> Response;
}

struct Server {
    ws: ws::Sender,
    registry: ClientRegistry
}

impl PacketHandler for Server {
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

pub fn run_server(host: &str, port: i16) {
    impl ws::Handler for Server {        
        fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
            println!("Received: {}", msg);

            if self.registry.add(&[String::from("test")], &self.ws) {
                let _ = self.ws.send("Test1");
            } else {
                let _ = self.ws.send("Test2");
            }

            self.registry.emit(String::from("test"), String::from("Testing"));
            let result = &self.handle_message(&msg);
            self.ws.send(serde_json::to_string(result).unwrap())
        }

        fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        }
    }

    ws::listen(format!("{}:{}", host, port), |out| Server { ws: out, registry: ClientRegistry::new() }).unwrap();
}
