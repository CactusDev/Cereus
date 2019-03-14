
use packet::*;
use std::collections::HashMap;

pub mod manager;
pub mod api;

pub type BuiltinCommandHandler = Fn(&Context, &api::CommandAPI) -> Context;

#[macro_export]
macro_rules! handler {
	($handler:expr) => {
		{
			// Don't really need to do any extra processing here.
			$crate::command::HandlerType::Only(Box::new($handler))
		}
	};
	($($key:expr => $handler:expr),+) => {
		{
			// Once again, we don't really need to do anything else here other than package it into the correct type.
			let mut subcommands: std::collections::HashMap<String, Box<$crate::command::HandlerType>> = std::collections::HashMap::new();
			$(
				subcommands.insert($key.to_string(), Box::new($handler));
			)+
			$crate::command::HandlerType::SubCommands(subcommands)
		}
	}
}

#[macro_export]
macro_rules! command {
	($name:expr, $($subcommand:expr => $handler:expr),+) => {
		{
			let mut command_data: std::collections::HashMap<String, Box<$crate::command::HandlerType>> = std::collections::HashMap::new();
			$(
				// And we know that our handlers have been parsed, so we can attempt to turn this into a functional
				// command structure that we can actually use.
				command_data.insert($subcommand.to_string(), Box::new($handler));
			)+
			if let None = command_data.get("default") {
				command_data.insert("default".to_string(), Box::new(handler!(|_context, _api| {
					// TODO: Make this more meaningful
					Context::message(vec! [ text!("Invalid argument!") ])
				})));
			}
			$crate::command::Command::new($name, command_data)
		}
	}
}

pub mod cactus;
pub mod command;
pub mod quote;
pub mod multi;

pub enum HandlerType {
	/// Handler type only has a default handler
	Only(Box<BuiltinCommandHandler>),
	/// Handler type contains named subcommands
	SubCommands(HashMap<String, Box<HandlerType>>)
}

/// Command is a processed command that has had it's entire command tree processed.
pub struct Command {
	pub name: String,
	pub commands: HashMap<String, Box<HandlerType>>
}

impl Command {

	pub fn new(name: &str, commands: HashMap<String, Box<HandlerType>>) -> Self {
		Command {
			name: name.to_string(),
			commands
		}
	}

	pub fn get_default_command_executor(&self) -> (usize, Option<&Box<BuiltinCommandHandler>>) {
		self.get_named_subcommand(vec! ["default".to_string()])
	}

	pub fn get_named_subcommand(&self, arguments: Vec<String>) -> (usize, Option<&Box<BuiltinCommandHandler>>) {
		match arguments.as_slice() {
			[first, remaining..] => {
				match self.commands.get(first) {
					Some(cmd) => {
						let cmd = cmd;
						let mut name_index = 1;
						let max_name_index = remaining.len();
						if name_index >= max_name_index {
							return (0, None);
						}

						let mut current_command = &**cmd;
						loop {
							let current_name = &remaining[name_index - 1];
							match current_command {
								HandlerType::Only(command) => return (name_index, Some(&command)),
								HandlerType::SubCommands(subcommands) => match subcommands.get(current_name) {
									Some(cmd) => current_command = cmd,
									None => return (0, None)
								}
							}
						}
					},
					None => return self.get_default_command_executor()
				}
			},
			_ => (0, None)
		}
	}
}
