
use handler::Handler;
use packet::*;

pub struct LoggingHandler;

impl LoggingHandler {

	pub fn new() ->  Self {
		LoggingHandler {}
	}
}

impl Handler for LoggingHandler {

	fn run(&self, context: &Context) -> Option<Packet> {
		println!("{:?}", context);
		None
	}
}
