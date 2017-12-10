
use ws::listen;

pub fn handle_server(port: i32) {
    if let Err(error) = listen("localhost:5000", |out| {
        move |msg| {
            println!("Got: {}", msg);
            out.send(msg)
        }
    }) {
        println!("Unable to create websocket server! {:?}", error);
    }
}
