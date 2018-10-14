
extern crate env_logger;
extern crate iron;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod web;
mod packet;
mod handler;
#[macro_use]
mod command;

use packet::*;

fn main() {
    env_logger::init().unwrap();

    command!("cactus",
    	"default" => handler!(|_context| {
    		Packet::Message { text: vec! [
    			text!("Ohai! I'm CactusBot! "),
    			emoji!("cactus")
    		], action: false }
    	}),
    	"github" => handler!(
    		"default" => |_context| {
    			Packet::Message { text: vec! [
    				text!("We're open source! Check it out at: "),
    				url!("https://github.com/CactusDev")
    			], action: false }
    		},
    		"cereus" => |_context| {
    			Packet::Message { text: vec! [
    				text!("Checkout Cereus at: "),
    				url!("https://github.com/CactusDev/Cereus")
    			], action: false }
    		}
    	)
    )

    // let mut w = web::WebServer::new("localhost", 1234);
    // w.listen();
}
