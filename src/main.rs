extern crate env_logger;
extern crate ws;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod packet;
mod server;
mod response_builder;

fn main() {
    env_logger::init().unwrap();

    server::handle_server("localhost", 5000);
}
