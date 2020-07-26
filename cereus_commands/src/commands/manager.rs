
use std::{
	collections::HashMap,
	convert::TryInto
};
use rand::{
	thread_rng,
	seq::SliceRandom
};

use regex::{Regex, Captures};
use crate::commands::{Command, APIHandler, api::CactusAPI};
use cereus_core::{
	ARGN_REGEX, ARGS_REGEX,
	types::{Packet, Context, Component, Role, string_components_to_string}
};

#[derive(Debug)]
enum DynamicCommandError {
	RequestError(reqwest::Error)
}

type ModifierFunction = dyn Fn(&String) -> String;

pub struct CommandManager {
	commands: HashMap<String, Command>,
	api: Box<dyn APIHandler>,
	argn_regex: Regex,
	args_regex: Regex,
	modifiers: HashMap<String, Box<ModifierFunction>>
}

impl CommandManager {

	pub fn new(api_base: &str) -> Self {
		CommandManager {
			commands: HashMap::new(),
			api: Box::new(CactusAPI::new(api_base)),
			argn_regex: ARGN_REGEX.clone(),
			args_regex: ARGS_REGEX.clone(),
			modifiers: {
				let mut modifiers: HashMap<String, Box<ModifierFunction>> = HashMap::new();

				modifiers.insert("upper".to_string(), Box::new(|s: &String| s.to_uppercase()));
				modifiers.insert("lower".to_string(), Box::new(|s: &String| s.to_lowercase()));

				modifiers.insert("title".to_string(), Box::new(|s: &String| {
				    let mut c = s.chars();
				    match c.next() {
				        None => String::new(),
				        Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase()
				    }
				}));

				modifiers.insert("reverse".to_string(), Box::new(|s| s.chars()
					.rev().collect::<String>()));

				modifiers.insert("tag".to_string(), Box::new(|s| {
					let (first, remaining) = s.split_at(1);
					if first == "@" {
						return remaining.to_string();
					}
					s.to_string()
				}));

				modifiers.insert("shuffle".to_string(), Box::new(|s| {
					let mut chars: Vec<char> = s.chars().collect();
					let slice = chars.as_mut_slice();
					let mut rng = thread_rng();
					slice.shuffle(&mut rng);

					slice.iter().map(|s| *s).collect::<String>()
				}));

				modifiers
			}
		}
	}

	pub fn add_command(&mut self, command: Command) {
		let name = &command.name.clone();
		self.commands.insert(name.to_string(), command);
	}

	fn modify(&self, argument: String, modifiers: Vec<String>) -> String {
		let mut argument = argument;
		for modifier in modifiers {
			match &self.modifiers.get(&modifier) {
				Some(ref function) => argument = function(&argument),
				None => continue
			}
		}
		argument
	}

	fn sub_argn(&self, args: Vec<Component>, matches: &Captures) -> Option<String> {
		let argn      = matches.get(1);
		let default   = matches.get(2);
		let modifiers = matches.get(3);

		if argn.is_none() {
			return None;
		}

		let argn = argn.unwrap().as_str().parse::<usize>().unwrap();

		// TODO: this could be optimized
		let result = if argn - 1 < args.len() {
			 args[argn - 1].to_string()
		} else {
			if let Some(default) = default {
				 default.as_str().to_string()
			} else {
				return None
			}
		};

		if let Some(modifiers) = modifiers {
			let modifiers = modifiers.as_str().split("|").skip(1)
				.map(|s| s.to_string()).collect::<Vec<String>>();
			// Attempt to modify the result
			return Some(self.modify(result, modifiers));
		}
		return Some(result);
	}

	fn sub_args(&self, args: Vec<Component>, matches: &Captures) -> Option<String> {
		let default   = matches.get(1);
		let modifiers = matches.get(2);

		let args = if args.len() > 0 {
			args
		} else {
			if let Some(default) = default {
                vec! [ Component::Text(default.as_str().to_string()) ]
			} else {
			    return None
			}
		};

		let result: String = args.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");

		if let Some(modifiers) = modifiers {
			let modifiers = modifiers.as_str().split("|").skip(1)
				.map(|s| s.to_string()).collect::<Vec<String>>();
			// Attempt to modify the result
			return Some(self.modify(result, modifiers));
		}
		return Some(result);
	}

	fn try_dynamic_command(&self, name: &str, context: &Context) -> Result<Context, DynamicCommandError> {
		// The name of the command should be the first component, so lets pull that out
		let response = self.api.get_command(&context.channel, name).map_err(|err| DynamicCommandError::RequestError(err))?;

		// Check the role of the command.
		if response.meta.role > context.role.clone().unwrap_or(Role::User) {
			// User is not able to run this command.
			return Ok(Context {
				packet: Packet::Message { text: vec![ text!("You do not have permission for this command!") ], action: false },
				channel: context.channel.clone(),
				user: None,
				role: None,
				target: context.user.clone(),
				service: None,
				count: None
			});
		}

		return Ok(Context {
			packet: Packet::Message { text: response.response, action: false },
			channel: response.channel,
			user: None,
			role: None,
			target: None,
			service: None,
			count: Some(response.meta.count.try_into().unwrap())
		})
	}

	fn fill_response_formatters(&self, context: &Context, input: Vec<Component>) -> Result<Context, ()> {
		match context.packet {
			Packet::Message { ref text, action } => {
				let mut filled_components: Vec<Component> = vec! [];

				for component in text {
					match component.clone() {
						Component::Text(ref text) => {
							let mut text = text.to_string();

							// Fill %USER% if we have it.
							if let Some(ref user) = &context.user {
								text = text.replace("%USER%", user);
							}
							text = text.replace("%COUNT%", &context.count.unwrap_or(0).to_string());
							// Next, fill the channel.
							text = text.replace("%CHANNEL%", &context.channel);

							// Then, args / argn
							if self.argn_regex.is_match(&text.clone()) {
								text = self.argn_regex.replace(&text, |caps: &Captures| self.sub_argn(input.clone(), caps.clone()).unwrap_or(String::new())).to_string();
							}

							if self.args_regex.is_match(&text.clone()) {
								text = self.args_regex.replace(&text, |caps: &Captures| self.sub_args(input.clone(), caps.clone()).unwrap_or(String::new())).to_string();
							}

							// Finally, lets store the component.
							filled_components.push(Component::Text(text));
						},
						_ => filled_components.push(component.clone())
					}
				}

				let mut finished_context = context.clone();
				finished_context.packet = Packet::Message { text: filled_components, action: action };
				Ok(finished_context)
			},
			_ => Err(())
		}
	}

	pub fn run_command(&self, context: &Context) -> Option<Context> {
		match context.packet {
			Packet::Message { ref text, action } => match text.as_slice() {
				[Component::Text(name), arguments @ ..] => match self.commands.get(&name.trim().replace("!", "")) {
					Some(handler) => {
						let mut args = string_components_to_string(arguments.to_vec());  // Can this be eliminated? @speed
						match args.clone().last() {
							Some(arg) => if arg != "default" {
								args.push("default".to_string());
							},
							_ => {}
						}
						match handler.get_named_subcommand(args) {
							(index, Some(handler)) => {
								let context = context.clone().cut(index);
								self.fill_response_formatters(&handler(&context, &self.api, context.get_packet_content(), action).merge(&context), context.get_packet_content()).ok()
							},
							(_, None) => None
						}
					},
					None => {
						match self.try_dynamic_command(&name.replace("!", ""), &context) {
							Ok(ctx) => self.fill_response_formatters(&ctx.merge(&context), context.clone().cut(1).get_packet_content()).ok(),
							Err(_) => Some(Context::message(vec! [ text!("Command not found.") ]))
						}
					}
				},
				_ => None
			},
			_ => None
		}
	}
}
