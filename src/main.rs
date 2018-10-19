
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

    let context = packet::Context {
        packet: packet::Packet::Message { text: vec! [text!("!cactus"), text!("github"), text!("cereus")], action: false },
        channel: "Stanley".to_string(),
        user: None,
        role: None,
        target: None,
        service: "Mixer".to_string()
    };

    println!("{:?}", manager.run_command(&context));
    // let test = things.get_named_subcommand(vec! ["github".to_string(), "cereus".to_string()]);
    // match test {
    // 	Some(t) => println!("WE HAVE IT"),
    // 	None    => println!("BIG SAD")
    // };

    // let mut w = web::WebServer::new("localhost", 1234);
    // w.listen();
}
