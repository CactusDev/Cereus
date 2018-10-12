
use std::collections::HashMap;

use handler::Handler;
use packet::*;

pub struct CommandHandler {
	prefix: String
}

impl CommandHandler {

	pub fn new(prefix: &str) -> Self {
		CommandHandler {
			prefix: prefix.to_string()
		}
	}
}

impl Handler for CommandHandler {

	fn run(&mut self, context: &mut Context) -> Option<Packet> {
		match context.clone().packet {
			Packet::Message { mut text, action } => {
				// Get the first segment, and attempt to find a matching command
				let first = text.pop().unwrap();
				if let Component::Text(name) = first {
					// Attempt to resolve it from the internal list
				} else {
					// Not a string name
					return None;
				}
				None
			},
			_ => None
		}
	}
}
