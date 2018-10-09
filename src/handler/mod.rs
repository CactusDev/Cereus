
use packet::*;
use std::vec::Vec;

mod command;

pub trait Handler {

	fn run(&mut self, context: Context) -> Option<Packet>;
}
