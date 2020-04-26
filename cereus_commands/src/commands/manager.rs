
use std::collections::HashMap;
use rand::{
	thread_rng,
	seq::SliceRandom
};

use regex::{Regex, Captures};
use crate::commands::{Command, api::CactusAPI};
use cereus_core::{
	ARGN_REGEX, ARGS_REGEX,
	types::{Packet, Context, Component, string_components_to_string}
};

#[derive(Debug)]
enum DynamicCommandError {
	RequestError(reqwest::Error)
}

type ModifierFunction = dyn Fn(&String) -> String;

pub struct CommandManager {
	commands: HashMap<String, Command>,
	api: CactusAPI,
	argn_regex: Regex,
	args_regex: Regex,
	modifiers: HashMap<String, Box<ModifierFunction>>
}

struct DynamicCommandMeta {
	count: u32
}

impl CommandManager {

	pub fn new(api_base: &str) -> Self {
		CommandManager {
			commands: HashMap::new(),
			api: CactusAPI::new(api_base),
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

	fn try_dynamic_command(&self, channel: &str, name: &str) -> Result<Context, DynamicCommandError> {
		// The name of the command should be the first component, so lets pull that out
		let response = self.api.get_command(channel, name).map_err(|err| DynamicCommandError::RequestError(err))?;
		// TODO: Finish count implementation
		return Ok(Context {
			packet: Packet::Message { text: response.response, action: false },
			channel: response.channel,
			user: None,
			role: None,
			target: None,
			service: None,
			count: None  // TODO
		})
	}

	fn fill_response_formatters(&self, context: &Context, input: Vec<Component>, meta: Option<DynamicCommandMeta>) -> Result<Context, ()> {
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
							// Attempt to fill the count. This is only present on dynamics.
							if let Some(ref meta) = meta {
								// Since we were given meta, we know this is a dynamic command,
								// meaning we have the count.
								text = text.replace("%COUNT%", &meta.count.to_string());
							}
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
			Packet::Message { ref text, action: _ } => match text.as_slice() {
				[Component::Text(name), arguments @ ..] => match self.commands.get(&name.trim().replace("!", "")) {
					Some(handler) => {
						//
						// HACK: This was put in place as a temp. solution to #22 (https://github.com/CactusDev/Cereus/issues/22)
						// and should definitely be removed as fast as possible.
						//
						// Test: test_no_arguments_are_passed_to_handler_when_final_argument_is_end_of_subcommand_resolution
						// Bug details: 
						// ============
						// If there is no argument after the final argument for subcommand resolution, the final subcommand
						// resolution argument (in the test case, it's "test") is passed into the handler as the first argument
						// of the command. This "solution" just forces the command to call the default handler in there, even
						// though the resolver system should be able to handle that its self.
						//
						// @mustfix @speed
						//// Can this be eliminated? @speed
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
								self.fill_response_formatters(&handler(&context, &self.api).merge(&context), context.get_packet_content(), None).ok()
							},
							(_, None) => None
						}
					},
					None => {
						match self.try_dynamic_command(&context.channel, &name.replace("!", "")) {
							Ok(ctx) => self.fill_response_formatters(&ctx.merge(&context), context.clone().cut(1).get_packet_content(), None).ok(),
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
