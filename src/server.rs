
use ws::{listen, CloseCode, Message, Sender, Handler, Result};
use json::parse;
use error_builder::generate_error;

pub fn handle_server(host: &str, port: i32) {

    struct Server {
        ws: Sender,
    }

    impl Handler for Server {
        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("Got: {}", msg);
            let parsed = parse(&msg.to_string());
            if parsed.is_err() {
                return self.ws.send(generate_error("Packets must be JSON!"))
            }
            // Basic checks to make sure we can actually parse this packet
            let unwrapped = parsed.unwrap();
            if unwrapped["type"].is_null() {
                return self.ws.send(generate_error("Must provide a packet type."));
            }
            println!("Stuff: {}", unwrapped["a"]);
            self.ws.send(msg)
        }

        fn on_close(&mut self, _: CloseCode, _: &str) {
            self.ws.shutdown().unwrap();
        }
    }

    listen(format!("{}:{}", host, port), |out| {
        Server {
            ws: out,
        }
    }).unwrap();
}
