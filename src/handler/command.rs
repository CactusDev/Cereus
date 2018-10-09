
use handler::Handler;
use packet::*;

pub struct CommandHandler {
	prefix: String
}

impl CommandHandler {

	pub fn new(prefix: &str) -> Self {
		CommandHandler {
			prefix: prefix.to_string()
		}
	}
}

impl Handler for CommandHandler {

	fn run(&mut self, context: Context) -> Option<Packet> {
		match context.packet {
			Packet::Message { text, action } => {
				for (i, component) in text.iter().enumerate() {
					// If this is the first component, then we need to check if the prefix is present.
					if i == 0 {
						if let Component::Text(message) = component {
							// Since the first component is text, we can actually validate this.
							if !message.starts_with(&self.prefix) {
								// Prefix is not present, get out.
								return None
							}
						} else {
							// If the first component is not text, then this cannot possibly be a command.
							// However, maybe at somepoint in the future for some silly reason we might want
							// to support emoji prefixes for commands. If we do, this is where it would go.
							//     :emoji_prefix
							//         - Innectic, 10/8/18
							return None
						}
					}
				}
				None
			},
			_ => None
		}
	}
}
