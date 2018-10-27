
use std::collections::HashMap;
use command::Command;

use packet::{Packet, Context, Component, string_components_to_string};

#[derive(Debug)]
enum DynamicCommandError {
	FirstElementMustBeText,
	RequestError(reqwest::Error)
}

pub struct CommandManager {
	commands: HashMap<String, Command>,
	client: reqwest::Client,
	api_base: String
}

impl CommandManager {

	pub fn new(api_base: &str) -> Self {
		CommandManager {
			commands: HashMap::new(),
			client:   reqwest::Client::new(),
			api_base: api_base.to_string()
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
