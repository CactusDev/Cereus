
use handler::Handler;
use packet::*;
use std::vec::Vec;

pub struct LoggingHandler;

impl LoggingHandler {

	pub fn new() ->  Self {
		LoggingHandler {}
	}
}

impl Handler for LoggingHandler {

	fn run(&self, context: &Context) -> Vec<Context> {
		println!("{:?}", context);
		Vec::new()
	}
}
