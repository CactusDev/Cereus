
use packet::*;
use std::vec::Vec;

pub mod command;
pub mod logging;
pub mod event;

pub trait Handler {
	fn run(&self, context: &Context) -> Option<Packet>;
}

pub struct HandlerHandler {
	handlers: Vec<Box<Handler>>
}

impl HandlerHandler {

	pub fn new(handlers: Vec<Box<Handler>>) -> Self {
		HandlerHandler {
			handlers
		}
	}

	pub fn handle(&self, context: &Context) -> Vec<Packet> {
		let mut packets: Vec<Packet> = Vec::new();

		for handler in &self.handlers {
			if let Some(packet) = handler.run(context) {
				packets.push(packet);
			}
		}
		packets
	}
}
