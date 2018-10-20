
extern crate env_logger;
extern crate iron;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod web;
pub mod packet;
pub mod handler;
#[macro_use]
pub mod command;

use packet::*;
use std::rc::Rc;

fn main() {
    env_logger::init().unwrap();

    let mut manager = command::manager::CommandManager::new();

    manager.add_command(command!("cactus",
    	"default" => handler!(|_context| {
    		Packet::Message { text: vec! [
    			text!("Ohai! I'm CactusBot! "),
    			emoji!("cactus")
    		], action: false }
    	}),
    	"github" => handler!(
    		"default" => handler!(|_context| {
    			Packet::Message { text: vec! [
    				text!("We're open source! Check it out at: "),
    				url!("https://github.com/CactusDev")
    			], action: false }
    		}),
    		"cereus" => handler!(|_context| {
    			Packet::Message { text: vec! [
    				text!("Checkout Cereus at: "),
    				url!("https://github.com/CactusDev/Cereus")
    			], action: false }
    		})
    	)
    ));

    let command_handler = handler::command::CommandHandler::new("!", manager);
    let handler_handler = handler::HandlerHandler::new(vec! [Box::new(command_handler)]);

    let context = packet::Context {
        packet: packet::Packet::Message { text: vec! [text!("cactus")], action: false },
        channel: "".to_string(),
        user: None,
        role: None,
        target: None,
        service: "".to_string()
    };
    println!("{:?}", handler_handler.handle(&context));

    // let mut w = web::WebServer::new("localhost", 1234);
    // w.listen();
}
