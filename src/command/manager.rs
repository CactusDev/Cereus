
use std::collections::HashMap;
use command::Command;

use packet::{Packet, Context, Component, string_components_to_string};

pub struct CommandManager {
	commands: HashMap<String, Command>
}

impl CommandManager {

	pub fn new() -> Self {
		CommandManager {
			commands: HashMap::new()
		}
	}

	pub fn add_command(&mut self, command: Command) {
		let name = &command.name.clone();
		self.commands.insert(name.to_string(), command);
	}

	pub fn run_command(&self, context: &Context) -> Option<Context> {
		if let Packet::Message { ref text, action: _ } = context.packet {
			return match text.split_first() {
				Some((name, arguments)) => {
					if let Component::Text(component_text) = name {
						let component_text = component_text.replace("!", "");
						return match self.commands.get(&component_text) {
							Some(cmd) => {
								return match cmd.get_named_subcommand(string_components_to_string(arguments.to_vec())) {
									Some(handler) => Some(handler(context)),
									None => None
								}
							},
							None => None
						}
					}
					None
				},
				None => None
			}
		}
		None
	}
}
