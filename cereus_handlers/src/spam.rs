
use cereus_core::types::{Context, Component, Packet, Role};
use crate::Handler;

pub struct SpamHandler;

impl SpamHandler {

	pub fn new() -> Self {
		SpamHandler {}
	}
}

fn caps_score(components: &Vec<Component>) -> u16 {
	let mut caps = 0;

	for component in components {
		if let Component::Text(text) = component {
			for c in text.chars() {
				if c.is_uppercase() {
					caps += 1;
				}
			}
		}
	}
	caps
}

fn count_emoji(components: &Vec<Component>) -> u16 {
	let mut emoji_components = 0;

	for component in components {
		if let Component::Emoji(_) = component {
			emoji_components += 1;
		}
	}

	emoji_components
}

fn contains_url(components: &Vec<Component>) -> bool {
	for component in components {
		if let Component::URL(_) = component {
			return true;
		}
	}
	false
}

impl Handler for SpamHandler {

	fn run(&self, context: &Context) -> Vec<Option<Context>> {
		// TODO: Query API for offence count and change action based on this
		// Should be something like /{channel}/offence/{offender}
		//
		// TODO: Pull these values from the API user configuration
		
		// If the user is a moderator or higher, then we don't care
		// TODO: Make sure we even care about the role via user config
		match context.role {
			Some(ref role) => match role {
				Role::Moderator | Role::Owner => return vec! [],
				Role::Subscriber => {},  // TODO: Check if subs can send links
				_ => {}
			},
			_ => {}
		}

		if let Packet::Message { ref text, action: _ } = context.packet {
			let caps = caps_score(&text);
			if caps > 16 {
				return vec! [
					Some(Context::target_message(context.user.clone(), vec! [
						text!("Please do not spam capital letters.")
					])),
					Some(Context {
						packet: Packet::Ban { duration: None },
						channel: String::new(),
						user: None,
						role: None,
						target: context.user.clone(),
						service: None,
                        count: None
					}),
					None
				]
			}

			let emoji = count_emoji(&text);
			if emoji > 6 {
				return vec! [
					Some(Context::target_message(context.user.clone(), vec! [
						text!("Please do not spam emoji.")
					])),
					Some(Context {
						packet: Packet::Ban { duration: None },
						channel: String::new(),
						user: None,
						role: None,
						target: context.user.clone(),
						service: None,
                        count: None
					}),
					None
				]
			}

			let has_url = contains_url(&text);
			if has_url {
				return vec! [
					Some(Context::target_message(context.user.clone(), vec! [
						text!("Please do not post URLs.")
					])),
					Some(Context {
						packet: Packet::Ban { duration: None },
						channel: String::new(),
						user: None,
						role: None,
						target: context.user.clone(),
						service: None,
                        count: None
					}),
					None
				]
			}
		}

		Vec::new()
	}
}
