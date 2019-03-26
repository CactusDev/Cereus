
use cereus_core::types::Context;
use std::vec::Vec;
use crate::Handler;

pub struct LoggingHandler;

impl LoggingHandler {

	pub fn new() ->  Self {
		LoggingHandler {}
	}
}

impl Handler for LoggingHandler {

	fn run(&self, context: &Context) -> Vec<Option<Context>> {
		println!("{:?}", context);
		Vec::new()
	}
}
