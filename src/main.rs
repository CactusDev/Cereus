
extern crate env_logger;
extern crate ws;

mod server;

fn main () {
    env_logger::init().unwrap();

    server::handle_server("localhost", 5000);
}
