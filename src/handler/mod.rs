
use packet::*;
use std::vec::Vec;

pub trait Handler {

	fn run(&mut self, context: Vec<Context>) -> Packet;
}
