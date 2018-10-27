
use std::collections::HashMap;
use command::Command;

use regex::Regex;

use packet::{Packet, Context, Component, string_components_to_string};

#[derive(Debug)]
enum DynamicCommandError {
	FirstElementMustBeText,
	RequestError(reqwest::Error)
}

pub struct CommandManager {
	commands: HashMap<String, Command>,
	client: reqwest::Client,
	api_base: String,
	argn_regex: Regex,
	args_regex: Regex
}

struct DynamicCommandMeta {
	count: usize
}

impl CommandManager {

	pub fn new(api_base: &str) -> Self {
		CommandManager {
			commands: HashMap::new(),
			client:   reqwest::Client::new(),
			api_base: api_base.to_string(),
			argn_regex: Regex::new(r#"%ARG(\d+)(?:=([^|]+))?(?:((?:\|\w+)+))?%"#).unwrap(),
			args_regex: Regex::new(r#"%ARGS(?:=([^|]+))?((?:\|\w+)+)?%"#).unwrap()
		}
	}

	pub fn add_command(&mut self, command: Command) {
		let name = &command.name.clone();
		self.commands.insert(name.to_string(), command);
	}

	fn get_api_url(&self, endpoint: &str) -> String {
		format!("{}{}", &self.api_base, endpoint)
	}

	fn try_dynamic_command(&self, channel: &str, name: &Component) -> Result<Context, DynamicCommandError> {
		// The name of the command should be the first component, so lets pull that out
		match name {
			Component::Text(ref command_name) => {
				let endpoint = format!("/channel/{}/command/{}", channel, command_name);
				let mut response = self.client.get(&self.get_api_url(&endpoint))
					.send().map_err(|err| DynamicCommandError::RequestError(err))?;
				match response.status().is_success() {
					true => match response.json() {
						Ok(result) => return Ok(result),
						Err(err) => return Err(DynamicCommandError::RequestError(err))
					},
					_ => return Err(DynamicCommandError::FirstElementMustBeText)
				}
			},
			_ => return Err(DynamicCommandError::FirstElementMustBeText)
		}
	}

	fn fill_response_formatters(&self, context: &Context, arguments: Vec<Context>, meta: Option<DynamicCommandMeta>) -> Option<Context> {
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
							// Finally, lets store the component.
							filled_components.push(Component::Text(text));
						},
						_ => filled_components.push(component.clone())
					}
				}
				None
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
									Some(handler) => return Some(handler(context)),
									None => return None
								}
							},
							None => {
								// If we don't have a builtin with this name, then we're
								// going to check the API / Cache to see if we have a
								// custom command or an alias by this name.
								match self.try_dynamic_command(&context.channel, name) {
									Ok(context) => return Some(context),
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
