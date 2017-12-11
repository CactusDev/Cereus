
extern crate env_logger;
extern crate ws;

extern crate json;

mod server;

fn main () {
    env_logger::init().unwrap();

    server::handle_server("localhost", 5000);
}
