
use command::Command;
use packet::{Component, Packet, Context};

pub fn create_command_command() -> Command {
	command!("command",
		"add" => handler!(|context, api| {
			match context.packet {
				Packet::Message { ref text, action: _ } => {
					let (name, response) = {
						match text.as_slice() {
							[_, _, name, rest..] => match name {
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
							text!("Command '!"),
							text!(name.trim()),
							text!("' has been added!")
						]),
						Err(e) => {
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
						[_, _, name, _rest..] => match name {
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
						Err(e) => {
							println!("could not remove command for {}: {:?}", &context.channel, e);
							Context::message(vec! [
								text!("Encountered error while removing command.")
							])
						}
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
