
extern crate env_logger;
extern crate iron;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod web;
mod packet;
mod handler;

fn main() {
    env_logger::init().unwrap();

    let mut w = web::WebServer::new("localhost", 1234);
    w.listen();
}
