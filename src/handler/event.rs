
use handler::Handler;
use packet::*;

// TODO: Caching
pub struct EventHandler;

impl EventHandler {

	pub fn new() -> Self {
		EventHandler {}
	}
}

impl Handler for EventHandler {

	fn run(&self, context: &Context) -> Vec<Option<Context>> {
		match context.packet {
			Packet::Event { ref kind } => match kind {
				Event::Start { new } if *new => {
					vec! [
						Some(Context::message(vec! [
							text!("Welcome to CactusBot. "),
							emoji!("cactus")])),
						Some(Context::message(vec! [
							text!("Type '!cactus help' for assistance.")
						]))
					]
				},
				Event::Start { new } if !new => {
					vec! [ Some(Context::message(vec! [
						text!("CactusBot activated. "),
						emoji!("cactus")
					])) ]
				}
				Event::Follow { success } if *success => {
					vec! [ Some(Context::message(vec! [
							text!("Thanks for the follow, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						])) ]
				},
				Event::Subscribe { streak } if *streak == 1 => {
					vec! [ Some(Context::message(vec! [
							text!("Thanks for subscribing, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						])) ]
				},
				Event::Subscribe { streak } if *streak > 1 => {
					vec! [ Some(Context::message(vec! [
							text!("Thanks for resubscribing for {} months, ", &streak.to_string()),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						])) ]
				},
				Event::Host { success } if *success => {
					vec! [ Some(Context::message(vec! [
							text!("Thanks for the host, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						])) ]
				},
				Event::Join { success } if *success => {
					vec! [ Some(Context::message(vec! [
							text!("Welcome, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						])) ]
				},
				Event::Join { success } if !*success => {
					vec! [ Some(Context::message(vec! [
							text!("Thanks for watching, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						])) ]
				},
				_ => vec! []
			},
			_ => vec! []
		}
	}
}
