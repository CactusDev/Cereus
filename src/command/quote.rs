
use command::Command;
use packet::{Packet, Context};

pub fn create_quote_command() -> Command {
    command!("quote",
        "default" => handler!(|context, api| {
            // Get a random quote
            let result = api.get_random_quote(&context.channel);
			match result {
				Ok(quote) => Context::message(quote.response),
				Err(_) => Context::message(vec! [ text!("No quote found!") ])
			}
        }),
        "add" => handler!(|context, api| {
        	match context.packet {
				Packet::Message { ref text, action: _ } => {
					let response = {
						match text.as_slice() {
							[_, _, response..] => response.to_vec(),
							_ => return Context::message(vec! [ text!("Invalid syntax! !quote add <response...>") ])
						}
					};

					let result = api.create_quote(&context.channel, response);
					match result {
						Ok(data) => Context::message(vec! [
							text!("Quote "),
							text!(data.id),
							text!(" has been added!")
						]),
						Err(_) => {
							Context::message(vec! [ text!("Could not add quote!") ])
						}
					}
				},
				_ => {
					println!("Got non-message packet to command handler.");
					Context::message(vec! [])
				}
			}
        })
    )
}
