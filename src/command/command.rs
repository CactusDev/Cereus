
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
								_ => return Context::message(vec! [])
							}
							_ => return Context::message(vec! [])
						}
					};

					let result = api.create_command(&context.channel, name, response);
					match result {
						Ok(()) => Context::message(vec! [
							text!("Command '!"),
							text!(name),
							text!("' has been added!")
						]),
						Err(e) => {
							println!("{:?}", e);
							Context::message(vec! [
								text!("DUN BROKE")
							])
						}
					}
				},
				_ => Context::message(vec! [ text!("DUN BROKE 2") ])
			}
		})
	)
}
