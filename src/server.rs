
use ws::{listen, CloseCode, Message, Sender, Handler, Result};
use json::parse;

pub fn handle_server(host: &str, port: i32) {

    struct Server {
        ws: Sender,
    }

    impl Handler for Server {
        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("Got: {}", msg);
            let parsed = parse(&msg.to_string());
            if parsed.is_err() {
                return self.ws.send("Uh oh!")
            }
            println!("Stuff: {}", parsed.unwrap()["a"]);
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
