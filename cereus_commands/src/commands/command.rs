
use crate::commands::Command;
use cereus_core::{
	COMMAND_PREFIX,
	types::{Component, Context, Command as TCommand}
};

fn make_command_string(commands: Vec<TCommand>) -> Option<String> {
	if commands.len() == 0 {
		return None
	}

	return Some(commands.iter().fold("".to_string(),
                  |a, b| if a.len() > 0 { a + ", " } else { a } + &b.name));
}

fn ident_to_role(role: &str) -> String {
	match role {
		"+" => "moderator".to_string(),
		"$" => "subscriber".to_string(),
		_ => "user".to_string()
	}
}

pub fn create_command_command() -> Command {
	command!("command",
		"add" => handler!(|context, api, text, _action| {
			let (role, name, response) = {
				match text.as_slice() {
					[name, rest @ ..] => match name {
						Component::Text(name) => {
							// Check if the first char of the name is a permission prefix
							let first = name.chars().next().unwrap();
							let first = first.to_string();
							match COMMAND_PREFIX.contains(&first) {
								// We do have it, that means this should be restricted in some form.
								true => (first.clone(), name.chars().skip(1).collect::<String>(), rest.to_vec()),
								false => ("".to_string(), name.clone(), rest.to_vec())
							}
						},
						_ => return Context::message(vec! [
							text!("Invalid syntax! !command add <name> <response...>")
						])
					},
					_ => return Context::message(vec! [])
				}
			};

			let role = ident_to_role(&role);
			let result = api.create_command(&context.channel, &name, response, &role);
			match result {
				Ok(()) => Context::message(vec! [
					text!(&format!("Command '!{}' ", &name)),
					text!("has been added!")
				]),
				Err(e) => {
					println!("{:?}", e);
					Context::message(vec! [
						text!("Command already exists!")
					])
				}
			}
		}),
		"remove" => handler!(|context, api, text, _action| {
			let name = match text.as_slice() {
				[name, _rest @ ..] => match name {
					Component::Text(name) => name,
					_ => return Context::message(vec! [
						text!("Invalid syntax! !command remove <name>")
					])
				},
				_ => return Context::message(vec! [])
			};

			let result = api.remove_command(&context.channel, &name);
			match result {
				Ok(()) => Context::message(vec! [
					text!("Command removed.")
				]),
				Err(_) =>
					Context::message(vec! [ text!("Command does not exist!") ])
			}
		}),
		"list" => handler!(|context, api, _text, _action| {
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
		"edit" => handler!(|context, api, text, _action| {
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
		}),
		"enable" => handler!(|context, api, text, _action| {
			let name = match text.as_slice() {
				[name, _remaining @ ..] => match name {
					Component::Text(name) => name,
					_ => return Context::message(vec! [ text!("Invalid syntax! !command enable <name>") ])
				},
				_ => return Context::message(vec! [])
			};

			let result = api.change_command_state(&context.channel, name, true);
			match result {
				Ok(data) => match data.previous_state {
					true => Context::message(vec! [ text!("Command is already enabled!") ]),
					false => Context::message(vec! [ text!("Command has been enabled!") ])
				}
				Err(_) => Context::message(vec! [ text!("Command does not exist!") ])
			}
		}),
		"disable" => handler!(|context, api, text, _action| {
			let name = match text.as_slice() {
				[name, _remaining @ ..] => match name {
					Component::Text(name) => name,
					_ => return Context::message(vec! [ text!("Invalid syntax! !command disable <name>") ])
				},
				_ => return Context::message(vec! [])
			};

			let result = api.change_command_state(&context.channel, name, false);
			match result {
				Ok(data) => match !data.previous_state {
					true => Context::message(vec! [ text!("Command is already disabled!") ]),
					false => Context::message(vec! [ text!("Command has been disabled!") ])
				}
				Err(_) => Context::message(vec! [ text!("Command does not exist!") ])
			}
		})
	)
}
