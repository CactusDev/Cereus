
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

	fn run(&self, context: &Context) -> Vec<Context> {
		match context.clone().packet {
			Packet::Message { text, action } => match self.manager.run_command(context) {
				Some(context) => vec! [ context ],
				None => vec! []
			},
			_ => vec! []
		}
	}
}
 