
use std::collections::HashMap;
use rand::{
	thread_rng,
	seq::SliceRandom
};

use command::Command;
use packet::{Packet, Context, Component, string_components_to_string};

use regex::{Regex, Captures};

#[derive(Debug)]
enum DynamicCommandError {
	FirstElementMustBeText,
	RequestError(reqwest::Error)
}

type ModifierFunction = Fn(&String) -> String;

pub struct CommandManager {
	commands: HashMap<String, Command>,
	client: reqwest::Client,
	api_base: String,
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
			client:   reqwest::Client::new(),
			api_base: api_base.to_string(),
			argn_regex: Regex::new(r#"%ARG(\d+)(?:=([^|]+))?(?:((?:\|\w+)+))?%"#).unwrap(),
			args_regex: Regex::new(r#"%ARGS(?:=([^|]+))?((?:\|\w+)+)?%"#).unwrap(),
			modifiers: {
				let mut modifiers: HashMap<String, Box<ModifierFunction>> = HashMap::new();

				modifiers.insert("upper".to_string(), Box::new(|s: &String| s.to_uppercase()));
				modifiers.insert("lower".to_string(), Box::new(|s: &String| s.to_lowercase()));

				modifiers.insert("title".to_string(), Box::new(|s: &String| {
					let mut chars = s.chars();
					match chars.next() {
						Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
						None => String::new()
					}
				}));

				modifiers.insert("reverse".to_string(), Box::new(|s| s.chars()
					.rev().collect::<String>()));

				modifiers.insert("tag".to_string(), Box::new(|s| {
					let (first, remaining) = s.split_at(0);
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

	pub fn get_api_url(&self, endpoint: &str) -> String {
		format!("{}/{}", &self.api_base, endpoint)
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
			println!("THAR BE NO THAANG");
			return None;
		}

		let argn = argn.unwrap().as_str().parse::<usize>().unwrap();

		let mut result: Option<String> = None;

		// TODO: this could be optimized
		println!("ARGN: {}, ARGS: {}, ALL ARGS: {:?}", argn - 1, args.len(), args);
		result = if argn - 1 < args.len() {
			Some(args[argn - 1].to_string())
		} else {
			if let Some(default) = default {
				Some(default.as_str().to_string())
			} else {
				None
			}
		};

		if result.is_none() {
			println!("RESULTO IS NO");
			return None;
		}

		if let Some(modifiers) = modifiers {
			println!("MERDIFER");
			let modifiers = modifiers.as_str().split("|").skip(1)
				.map(|s| s.to_string()).collect::<Vec<String>>();
			// Attempt to modify the result
			return Some(self.modify(result.unwrap(), modifiers));
		}
		println!("THANG {:?}", result);
		return result;
	}

	fn sub_args(&self, args: Vec<Component>, matches: &Captures) -> Option<String> {
		let default   = matches.get(1);
		let modifiers = matches.get(2);
		println!("RSEITNRST");

		// Check if we were provided any arguments
		if args.len() == 0 {
			println!("THE NOTHING");
			// Since we weren't provided anything, there's nothing we can do here
			return None;
		}

		let result: String = args.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");

		if let Some(modifiers) = modifiers {
			let modifiers = modifiers.as_str().split("|").skip(1)
				.map(|s| s.to_string()).collect::<Vec<String>>();
			// Attempt to modify the result
			println!("MODIFY");
			return Some(self.modify(result, modifiers));
		}
		println!("NOIFY");
		return Some(result);
	}

	fn try_dynamic_command(&self, channel: &str, name: &Component) -> Result<Context, DynamicCommandError> {
		// The name of the command should be the first component, so lets pull that out
		match name {
			Component::Text(ref command_name) => {
				let endpoint = format!("channel/{}/command/{}", channel, command_name);
				let mut response = self.client.get(&self.get_api_url(&endpoint))
					.send().map_err(|err| DynamicCommandError::RequestError(err))?;
				match response.status().is_success() {
					true => match response.json::<Context>() {
						Ok(result) => {
							let meta = DynamicCommandMeta { count: result.count.unwrap_or(0) };
							return Ok(result)
						},
						Err(err) => return Err(DynamicCommandError::RequestError(err))
					},
					_ => return Err(DynamicCommandError::FirstElementMustBeText)
				}
			},
			_ => return Err(DynamicCommandError::FirstElementMustBeText)
		}
	}

	fn fill_response_formatters(&self, context: &Context, input: Vec<Component>, meta: Option<DynamicCommandMeta>) -> Option<Context> {
		match context.packet {
			Packet::Message { ref text, action } => {
				let mut filled_components: Vec<Component> = vec! [];
				println!("HERE BE TEXT: {:?}", text);

				if let Some((_command_name, args)) = input.split_first() {
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
								if let Some(caps) = self.argn_regex.captures(&text.clone()) {
									text = self.argn_regex.replace(&text, |caps: &Captures| self.sub_argn(args.to_vec(), caps.clone()).unwrap_or(String::new())).to_string();
								}

								if let Some(caps) = self.args_regex.captures(&text.clone()) {
									text = self.args_regex.replace(&text, |caps: &Captures| self.sub_args(args.to_vec(), caps.clone()).unwrap_or(String::new())).to_string();
								}

								// Finally, lets store the component.
								filled_components.push(Component::Text(text));
							},
							_ => filled_components.push(component.clone())
						}
					}
				}

				let mut finished_context = context.clone();
				finished_context.packet = Packet::Message { text: filled_components, action: false };  // TODO: This somehow needs to be passed down
				return Some(finished_context);
			},
			_ => None
		}
	}

	pub fn run_command(&self, context: &Context) -> Option<Context> {
		if let Packet::Message { ref text, action: _ } = context.packet {
			return match text.split_first() {
				Some((name, arguments)) => {
					if let Component::Text(component_text) = name {
						let component_text = component_text.replace("!", "");
						match self.commands.get(&component_text) {
							Some(cmd) => {
								match cmd.get_named_subcommand(string_components_to_string(arguments.to_vec())) {
									Some(handler) => return self.fill_response_formatters(&handler(context).merge(context), text.to_vec(), None),
									None => return None
								}
							},
							None => {
								// If we don't have a builtin with this name, then we're
								// going to check the API to see if we have a
								// custom command or an alias by this name.
								match self.try_dynamic_command(&context.channel, name) {
									Ok(ctx) => self.fill_response_formatters(&ctx.merge(context), text.to_vec(), None),
									Err(err) => {
										println!("Could not run command: {:?}", err);
										return None;
									}
								};
							}
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
