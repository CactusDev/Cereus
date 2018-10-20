
use std::{
	collections::HashMap
};

use command::manager::CommandManager;
use handler::Handler;
use packet::*;

pub struct CommandHandler {
	prefix:  String,
	manager: CommandManager
}

impl CommandHandler {

	pub fn new(prefix: &str, manager: CommandManager) -> Self {
		CommandHandler {
			prefix: prefix.to_string(),
			manager
		}
	}
}

impl Handler for CommandHandler {

	fn run(&self, context: &Context) -> Option<Packet> {
		match context.clone().packet {
			Packet::Message { text, action } => self.manager.run_command(context),
			_ => None
		}
	}
}
 