
use packet::*;
use handler::Handler;

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

	fn run(&self, context: &Context) -> Vec<Context> {
		// TODO: Query API for offence count and change action based on this
		// Should be something like /{channel}/offence/{offender}
		//
		// TODO: Pull these values from the API user configuration

		if let Packet::Message { ref text, action: _ } = context.packet {
			let caps = caps_score(&text);
			if caps > 16 {
				return vec! [
					Context::message(vec! [
						text!("Please do not spam capital letters.")
					]),
					Context {
						packet: Packet::Ban { duration: None },
						channel: String::new(),
						user: None,
						role: None,
						target: None,
						service: String::new()
					}
				]
			}

			let emoji = count_emoji(&text);
			if emoji > 6 {
				return vec! [
					Context::message(vec! [
						text!("Please do not spam emoji.")
					]),
					Context {
						packet: Packet::Ban { duration: None },
						channel: String::new(),
						user: None,
						role: None,
						target: None,
						service: String::new()
					}
				]
			}

			let has_url = contains_url(&text);
			if has_url {
				return vec! [
					Context::message(vec! [
						text!("Please do not post URLs.")
					]),
					Context {
						packet: Packet::Ban { duration: None },
						channel: String::new(),
						user: None,
						role: None,
						target: None,
						service: String::new()
					}
				]
			}
		}

		Vec::new()
	}
}
