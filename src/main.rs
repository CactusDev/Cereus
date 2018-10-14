
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

    // command!("cactus",
    // 	"default" => handler!(|_context| {
    // 		Packet::Message { text: vec! [
    // 			text!("Ohai! I'm CactusBot! "),
    // 			emoji!("cactus")
    // 		], action: false }
    // 	}),
    // 	"github" => subcommand!(
    // 		"defaultT" => handler!(|_context| {
    // 			Packet::Message { text: vec! [
    // 				text!("We're open source! Check it out at: "),
    // 				url!("https://github.com/CactusDev")
    // 			], action: false }
    // 		}),
    // 		"test" => subcommand!(  // cactus github test
    // 			"default" => handler!(|_context| {
    // 				Packet::Message { text: vec! [
    // 					text!("HELLO WORLD I EXIST PLEASE HELP ME")
    // 				], action: false }
    // 			})
    // 		)
    // 	)
    // )

 	// let test = handler!("default", |_context| {
  //   		Packet::Message { text: vec! [
  //   			text!("Ohai! I'm CactusBot! "),
  //   			emoji!("cactus")
  //   		], action: false }
  //   	});

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

 //    let values = command!(
 //    	"default" => |_context| {
	// 		Packet::Message { text: vec! [
	// 			text!("Ohai! I'm CactusBot! "),
	// 			emoji!("cactus")
	// 		], action: false }
	// 	},
	// 	"docs" => |_context| {
	// 		Packet::Message { text: vec! [
	// 			text!("Check out my documentation at "),
	// 			url!("https://cactusbot.rtfd.org"),
	// 			text!(".")
	// 		], action: false }
	// 	},
	// 	"twitter" => |_context| {
	// 		Packet::Message { text: vec! [
	// 			text!("You can follow the team behind CactusBot at: "),
	// 			url!("https://twitter.com/CactusDevTeam")
	// 		], action: false }
	// 	},
	// 	"github" => |_context| {
	// 		Packet::Message { text: vec! [
	// 			text!("We're opensource! Check us out at: "),
	// 			url!("https://github.com/CactusDev")
	// 		], action: false }
	// 	},
	// 	"help" => |_context| {
	// 		Packet::Message { text: vec! [
	// 			text!("Try our docs (!cactus docs). If you're still having issues, tweet us! (!cactus twitter)")
	// 		], action: false }
	// 	}
	// );
    // let mut w = web::WebServer::new("localhost", 1234);
    // w.listen();
}
