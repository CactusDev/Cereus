
use crate::commands::Command;
use cereus_core::types::{Packet, Context, Component};

pub fn create_quote_command() -> Command {
    command!("quote",
        "default" => handler!(|context, api| {
        	match context.packet {
        		Packet::Message { ref text, action: _ } => {
        			let id = match text.as_slice() {
        				[id, _rest @ ..] => match id {
        					Component::Text(id) => id,
        					_ => return Context::message(vec! [ text!("Invalid syntax! !quote [id]") ])
        				},
        				_ => {
        					let result = api.get_random_quote(&context.channel);
							match result {
								Ok(quote) => return Context::message(quote.response),
								Err(_) => return Context::message(vec! [ text!("No quote found!") ])
							}
						}
        			};

        			match api.get_quote(&context.channel, &id) {
        				Ok(quote) => return Context::message(quote.response),
        				Err(_) => return Context::message(vec! [ text!("Invalid quote id!") ])
        			};
        		},
				_ => {
					println!("Got non-message packet to command handler.");
					Context::message(vec! [])
				}
        	}
        }),
        "add" => handler!(|context, api| {
        	match context.packet {
				Packet::Message { ref text, action: _ } => {
					let result = api.create_quote(&context.channel, text.to_vec());
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
        }),
        "remove" => handler!(|context, api| {
        	match context.packet {
        		Packet::Message { ref text, action: _ } => {
        			let id = match text.as_slice() {
        				[id, _rest @ ..] => match id {
        					Component::Text(id) => id,
        					_ => return Context::message(vec! [ text!("Invalid syntax! !quote remove <id>") ])
        				},
        				_ => return Context::message(vec! [])
        			};

        			match api.remove_quote(&context.channel, &id) {
        				Ok(()) => Context::message(vec! [ text!("Quote removed!") ]),
        				Err(_) => Context::message(vec! [ text!("Quote not found.") ])
        			}
        		},
				_ => {
					println!("Got non-message packet to command handler.");
					Context::message(vec! [])
				}
        	}
        }),
        "edit" => handler!(|context, api| {
            match context.packet {
                Packet::Message { ref text, action: _ } => {
                    if let Some((id, quote)) = text.split_first() {
                        if let Component::Text(id) = id {
                            return match api.edit_quote(&context.channel, &id, quote.to_vec()) {
                                Ok(()) => Context::message(vec! [ text!("Quote"), text!(id), text!("has been edited!") ]),
                                Err(_) => Context::message(vec! [ text!("Quote not found!") ])
                            };
                        }
                    }
                    return Context::message(vec! [ text!("Invalid syntax! !quote edit <id> <quote>") ]);
                },
                _ => {
                    println!("Got non-message packet to command handler.");
                    Context::message(vec! [])
                }
            }
        })
    )
}
