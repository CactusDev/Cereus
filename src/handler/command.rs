
use command::manager::CommandManager;
use handler::Handler;
use packet::*;

pub struct CommandHandler {
	prefix:  String,
	manager: CommandManager
}

impl CommandHandler {

	pub fn new(prefix: &str, manager: CommandManager) -> Self {
		CommandHandler {
			prefix: prefix.to_string(),
			manager
		}
	}
}

impl Handler for CommandHandler {

	fn run(&self, context: &Context) -> Vec<Option<Context>> {
		match context.clone().packet {
			Packet::Message { text, action: _ } => {
				// Check if the first text starts with the prefix
				let first = &text[0];
				if let Component::Text(text) = first {
					if !text.starts_with(&self.prefix) {
						return vec! [];
					}
					return match self.manager.run_command(context) {
						Some(context) => vec! [ Some(context) ],  // TODO: Multi-return command context
						None => vec! []
					};
				}
				vec! []
			},
			_ => vec! []
		}
	}
}
 