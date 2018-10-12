
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

    let values = command!(
    	"default" => |_context| {
			Packet::Message { text: vec! [
				Component::Text("Ohai! I'm CactusBot".to_string())
			], action: false }
		},
		"docs" => |_context| {
			Packet::Message { text: vec! [
				Component::Text("Check out my documentation at ".to_string()),
				Component::URL("https://cactusbot.rtfd.org".to_string()),
				Component::Text(".".to_string())
			], action: false }
		},
		"twitter" => |_context| {
			Packet::Message { text: vec! [
				Component::Text("You can follow the team behind CactusBot at: ".to_string()),
				Component::URL("https://twitter.com/CactusDevTeam".to_string())
			], action: false }
		},
		"github" => |_context| {
			Packet::Message { text: vec! [
				Component::Text("We're opensource! Check us out at: ".to_string()),
				Component::URL("https://github.com/CactusDev".to_string())
			], action: false }
		},
		"help" => |_context| {
			Packet::Message { text: vec! [
				Component::Text("Try our docs (!cactus docs). If you're still having issues, tweet us! (!cactus twitter)".to_string())
			], action: false }
		}
	);
	println!("{:?}", values.get("default").unwrap()("".to_string()));
    // let mut w = web::WebServer::new("localhost", 1234);
    // w.listen();
}
