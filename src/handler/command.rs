
use std::{
	rc::Rc,
	collections::HashMap
};

use command::manager::CommandManager;
use handler::Handler;
use packet::*;

pub struct CommandHandler {
	prefix:  String,
	manager: Rc<CommandManager>
}

impl CommandHandler {

	pub fn new(prefix: &str, manager: Rc<CommandManager>) -> Self {
		CommandHandler {
			prefix: prefix.to_string(),
			manager
		}
	}
}

impl Handler for CommandHandler {

	fn run(&mut self, context: &mut Context) -> Option<Packet> {
		match context.clone().packet {
			Packet::Message { text, action } => match Rc::get_mut(&mut self.manager) {
				Some(manager) => manager.run_command(context),
				None          => None  // TODO: We should error here, instead of None
			},
			_ => None
		}
	}
}
 