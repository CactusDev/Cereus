
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
pub mod trust;

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
		match arguments.split_first() {
			Some((first, remaining)) => {
				let current_command = self.commands.get(first);
				if let None = current_command {
					return self.get_default_command_executor();
				}
				let mut current_command: &HandlerType = current_command.unwrap();
				let mut name_index = 1;
				let max_name_index = remaining.len();

				loop {
					if max_name_index == 0 || name_index > max_name_index {
						// If we don't have anything else that we can look at, then what we have must be the final
						// thing that we're expecting.
						// However, if it's still a subcommand type, then we'll just give nothing back.
						return match current_command {
							HandlerType::SubCommands(sub) => {
								match sub.get("default") {
									Some(cmd) => {
										let cmd: &HandlerType = &**cmd;
										match cmd {
											HandlerType::Only(cmd) => (name_index, Some(&cmd)),
											_ => (name_index, None)
										}
									},
									None => (name_index, None)
								}
							},
							HandlerType::Only(cmd) => (name_index, Some(cmd))
						}
					}

					let current_name: &str = &remaining[name_index - 1];
					name_index += 1;

					match current_command {
						HandlerType::SubCommands(sub) => match sub.get(current_name) {
							Some(cmd) => current_command = cmd,
							None => match sub.get("default") {
								Some(cmd) => current_command = cmd,
								None => return (name_index, None)
							}
						},
						HandlerType::Only(cmd) => return (name_index, Some(cmd))
					}
				}
			},
			None => {
				// If we don't have any arguments, give the default handler.
				self.get_default_command_executor()
			}
		}
	}
}