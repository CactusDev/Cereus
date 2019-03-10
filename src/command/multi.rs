
use command::Command;
use packet::{Component, Packet, Context};

const BASE: &str = "https://multi.raredrop.co";
const VALID_SERVICES: [&str; 4] = ["t", "m", "y", "s"];

pub fn create_multi_command() -> Command {
	command!("multi",
		"default" => handler!(|context, _api| {
			match context.packet {
				Packet::Message { ref text, action: _ } => {
					let args = match text.as_slice() {
						[_, rest..] => rest.to_vec(),
						_ => return Context::message(vec! [ text!("Invalid syntax! !multi <service:channel...>") ])
					};

					if args.len() == 0 {
						return Context::message(vec! [ text!("Invalid syntax! !multi <service:channel...>") ])
					}

					let mut stream = String::new();
					for arg in &args {
						if let Component::Text(arg) = arg {
							let split = arg.split(":").collect::<Vec<&str>>();
							if split.len() != 2 {
								return Context::message(vec! [ text!("Invalid syntax! !multi <service:channel...>") ])
							}

							let (service, channel) = (split[0], split[1]);
							// Validate service
							if !VALID_SERVICES.contains(&service) {
								return Context::message(vec! [ text!("Invalid service '"), text!(&service), text!("'!") ])
							}
							stream += &format!("/{}{}", service, channel);
						} else {
							return Context::message(vec! [ text!("Invalid syntax! Only text can be used as a channel.") ])
						}
					}
					let stream = format!("{}{}", BASE, stream);
					Context::message(vec! [ text!(stream) ])
				},
				_ => {
					println!("Got non-message packet to command handler.");
					Context::message(vec! [])
				}
			}
		})
	)
}
