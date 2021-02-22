
use crate::commands::Command;
use cereus_core::types::{Component, Context};

const BASE: &str = "https://multi.raredrop.co";
const VALID_SERVICES: [&str; 4] = ["t", "m", "y", "s"];

pub fn create_multi_command() -> Command {
	command!("multi",
		"default" => handler!(|_context, _api, text, _action| {
			if text.len() == 0 {
				return Context::message(vec! [ text!("Invalid syntax! !multi <service:channel...>") ])
			}

			let mut stream = String::new();
			for arg in text {
				if let Component::Text(arg) = arg {
					let split = arg.split(":").collect::<Vec<&str>>();
					if split.len() != 2 {
						return Context::message(vec! [ text!("Invalid syntax! !multi <service:channel...>") ])
					}

					let (service, channel) = {
						let s = split[0];
						(s.chars().nth(0).unwrap_or('_').to_string(), split[1])
					};
					// Validate service
					if !VALID_SERVICES.contains(&&*service) {
						return Context::message(vec! [ text!("Invalid service '"), text!(&service), text!("'!") ])
					}
					stream += &format!("/{}{}", service, channel);
				} else {
					return Context::message(vec! [ text!("Invalid syntax! Only text can be used as a channel.") ])
				}
			}
			let stream = format!("{}{}", BASE, stream);
			Context::message(vec! [ text!(stream) ])
		})
	)
}
