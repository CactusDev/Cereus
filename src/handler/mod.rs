
use packet::*;
use std::vec::Vec;

pub mod command;

pub trait Handler {

	fn run(&mut self, context: &mut Context) -> Option<Packet>;
}
