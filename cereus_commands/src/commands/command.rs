
use crate::commands::Command;
use cereus_core::types::{Component, Packet, Context, Command as TCommand};

fn make_command_string(commands: Vec<TCommand>) -> Option<String> {
	if commands.len() == 0 {
		return None
	}

	return Some(commands.iter().fold("".to_string(),
                  |a, b| if a.len() > 0 { a + ", " } else { a } + &b.name));
}

pub fn create_command_command() -> Command {
	command!("command",
		"add" => handler!(|context, api| {
			match context.packet {
				Packet::Message { ref text, action: _ } => {
					let (name, response) = {
						match text.as_slice() {
							[name, rest @ ..] => match name {
								Component::Text(name) => (name, rest.to_vec()),
								_ => return Context::message(vec! [
									text!("Invalid syntax! !command add <name> <response...>")
								])
							},
							_ => return Context::message(vec! [])
						}
					};

					let result = api.create_command(&context.channel, name, response);
					match result {
						Ok(()) => Context::message(vec! [
							text!(&format!("Command '!{}' ", name)),
							text!("has been added!")
						]),
						Err(e) => {
							println!("{:?}", e);
							Context::message(vec! [
								text!("Command already exists!")
							])
						}
					}
				},
				_ => {
					println!("Got non-message packet to command handler.");
					Context::message(vec! [])
				}
			}
		}),
		"remove" => handler!(|context, api| {
			match context.packet {
				Packet::Message { ref text, action: _ } => {
					let name = match text.as_slice() {
						[name, _rest @ ..] => match name {
							Component::Text(name) => name,
							_ => return Context::message(vec! [
								text!("Invalid syntax! !command remove <name>")
							])
						},
						_ => return Context::message(vec! [])
					};

					let result = api.remove_command(&context.channel, name);
					match result {
						Ok(()) => Context::message(vec! [
							text!("Command removed.")
						]),
						Err(_) =>
							Context::message(vec! [ text!("Command does not exist!") ])
					}
				},
				_ => {
					println!("Got non-message packet to command handler.");
					Context::message(vec! [])
				}
			}
		}),
		"list" => handler!(|context, api| {
			let result = api.list_command(&context.channel);
			let response = match result {
				Ok(commands) => make_command_string(commands),
				Err(_) => None
			};
			match response {
				Some(response) => Context::message(vec! [
					text!("Enabled commands: "),
					text!(response)
				]),
				None => Context::message(vec![ text!("No commands found!") ])
			}
		}),
		"edit" => handler!(|context, api| {
			match context.packet {
				Packet::Message { ref text, action: _ } => {
					let (name, response) = match text.as_slice() {
						[name, rest @ ..] => match name {
							Component::Text(name) => (name, rest.to_vec()),
							_ => return Context::message(vec! [
								text!("Invalid syntax! !command edit <name> <response...>")
							])
						},
						_ => return Context::message(vec! [])
					};

					let result = api.edit_command(&context.channel, name, response);
					match result {
						Ok(()) => Context::message(vec! [
							text!("Command has been updated!")
						]),
						Err(_) => Context::message(vec! [ text!("Command does not exist!") ])
					}
				},
				_ => {
					println!("Got non-message packet to command handler.");
					Context::message(vec! [])
				}
			}
		})
	)
}
