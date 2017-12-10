
use ws::listen;

pub fn handle_server(host: &str, port: i32) {
    if let Err(error) = listen(format!("{}:{}", host, port), |out| {
        move |msg| {
            println!("Got: {}", msg);
            out.send(msg)
        }
    }) {
        println!("Unable to create websocket server! {:?}", error);
    }
}
