
use packet::*;
use std::vec::Vec;

pub mod command;
pub mod logging;
pub mod event;

pub trait Handler {
	fn run(&self, context: &Context) -> Vec<Context>;
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

	pub fn handle(&self, context: &Context) -> Vec<Context> {
		let mut contexts: Vec<Context> = Vec::new();

		for handler in &self.handlers {
			for context in handler.run(context) {
				// TODO: Check if this is a stop context, so we know when to stop
				contexts.push(context);
			}
		}
		contexts
	}
}
