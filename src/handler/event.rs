
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

	fn run(&self, context: &Context) -> Option<Packet> {
		match context.packet {
			Packet::Event { ref kind } => match kind {
				Event::Start { new } => {
					Some(Packet::Message { text: if *new {
						vec! [
								text!("Welcome to CactusBot. "),
								emoji!("cactus"),
								text!(" Type '!cactus help' for assistance.")
							]
						} else {
							vec! [
								text!("CactusBot activated. "),
								emoji!("cactus")
							]
						}, action: false
					})
				},
				Event::Follow { success } if *success => {
					Some(Packet::Message { text: vec! [
							text!("Thanks for the follow, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						], action: false
					})
				},
				Event::Subscribe { streak } if *streak == 1 => {
					Some(Packet::Message { text: vec! [
							text!("Thanks for subscribing, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						], action: false
					})
				},
				Event::Subscribe { streak } if *streak > 1 => {
					Some(Packet::Message { text: vec! [
							text!("Thanks for resubscribing for {} months, ", &streak.to_string()),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						], action: false
					})
				},
				Event::Host { success } if *success => {
					Some(Packet::Message { text: vec! [
							text!("Thanks for the host, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						], action: false
					})
				},
				Event::Join { success } if *success => {
					Some(Packet::Message { text: vec! [
							text!("Welcome, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						], action: false
					})
				},
				Event::Join { success } if !*success => {
					Some(Packet::Message { text: vec! [
							text!("Thanks for watching, "),
							tag!(context.user.clone().unwrap_or("Unknown".to_string())),
							text!("!")
						], action: false
					})
				},
				_ => None
			},
			_ => None
		}
	}
}
