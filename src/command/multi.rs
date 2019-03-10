
use command::Command;
use packet::{Component, Packet, Context};

const BASE: &str = "https://multi.raredrop.co";

pub fn create_multi_command() -> Command {
	command!("multi",
		"default" => handler!(|context, _api| {
			match context.packet {
				Packet::Message { ref text, action: _ } => {
					let args = match text.as_slice() {
						[_, rest..] => rest.to_vec(),
						_ => return Context::message(vec! [ text!("Invalid syntax! !multi <service:channel...>") ])
					};

					let mut stream = String::new();
					for arg in &args {
						// TODO: Should probably make an error for this.
						if let Component::Text(arg) = arg {
							let split = arg.split(":").collect::<Vec<&str>>();
							if split.len() != 2 {
								// TODO: Maybe put an error message here?
								continue;
							}

							let (service, channel) = (split[0], split[1]);
							stream += &format!("/{}{}", service, channel);
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
