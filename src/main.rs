
extern crate env_logger;
extern crate ws;

#[macro_use]
extern crate json;

mod server;
mod error_builder;

fn main () {
    env_logger::init().unwrap();

    server::handle_server("localhost", 5000);
}
