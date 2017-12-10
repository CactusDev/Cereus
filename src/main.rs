
extern crate env_logger;
extern crate ws;

use std::thread;

mod server;

fn main () {
    env_logger::init().unwrap();

    server::handle_server(5000);
}
