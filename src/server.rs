use ws;
use serde_json;

use packet;
use response_builder::Response;


pub fn handle_server(host: &str, port: i16) {

    struct Server {
        ws: ws::Sender,
    }

    impl ws::Handler for Server {
        fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {

            println!("Received: {}", msg);

            let build_response = |msg: ws::Message| {
                let parsed: serde_json::Result<packet::Context> =
                    serde_json::from_str(&msg.to_string());

                if parsed.is_err() {
                    return Response::Error("Invalid context.".to_string());
                }

                let deserialized = parsed.unwrap();
                println!("{:?}", deserialized);

                return Response::Success(serde_json::to_value(deserialized).unwrap());
            };

            let response = build_response(msg);

            self.ws.send(serde_json::to_string(&response).unwrap())
        }

        fn on_close(&mut self, _: ws::CloseCode, _: &str) {
            self.ws.shutdown().unwrap();
        }
    }

    ws::listen(format!("{}:{}", host, port), |out| Server { ws: out }).unwrap();
}
