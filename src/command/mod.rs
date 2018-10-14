
use packet::*;
use std::collections::HashMap;

pub type BuiltinCommandHandler = Fn(Context) -> Packet;

pub enum HandlerType {
	/// Handler type only has a default handler
	Only(Box<BuiltinCommandHandler>),
	/// Handler type contains named subcommands
	SubCommands(HashMap<String, Box<BuiltinCommandHandler>>)
}

#[macro_export]
macro_rules! command {
	// ($name:expr, $($key:expr => $value:expr),+) => {
	// 	{
	// 		let mut handlers: std::collections::HashMap<&str, $crate::command::BuiltinCommandHandler> = std::collections::HashMap::new();
	// 		$(handlers.insert($key, Box::new($value));)+
	// 		handlers
	// 	}
	// }
	($name:expr, $($subcommand:expr => $handler:expr),+) => {
		{
			$(
				// And we know that our handlers have been parsed, so we can attempt to turn this into a functional
				// command structure that we can actually use.
				println!("{} {}", $name, $subcommand);

				match $handler {
					$crate::command::HandlerType::Only(_) => println!("1"),
					$crate::command::HandlerType::SubCommands(_) => println!("2")
				};
			)+
		}
	}
}

#[macro_export]
macro_rules! handler {
	// If the handler macro sees just a string and a function (BuiltinCommandHandler), then this is an `Only`.
	($handler:expr) => {
		{
			// Don't really need to do any extra processing here.
			$crate::command::HandlerType::Only(Box::new($handler))
		}
	};
	// However, if it sees a name, and then a map of string -> functino (BuiltinCommandHandler), then we're handling subcommands.
	($($key:expr => $handler:expr),+) => {
		{
			$(println!("{}", $key);)+
			
			// Once again, we don't really need to do anything else here other than package it into the correct type.
			let mut subcommands: std::collections::HashMap<String, Box<$crate::command::BuiltinCommandHandler>> = std::collections::HashMap::new();
			$(
				subcommands.insert($key.to_string(), Box::new($handler));
			)+
			$crate::command::HandlerType::SubCommands(subcommands)
		}
	}
}

#[macro_export]
macro_rules! url {
	($url:expr) => {
		Component::URL($url.to_string())
	}
}

#[macro_export]
macro_rules! text {
	($text:expr) => {
		Component::Text($text.to_string())
	}
}

#[macro_export]
macro_rules! emoji {
	($emoji:expr) => {
		Component::Emoji($emoji.to_string())
	}
}

#[macro_export]
macro_rules! tag {
	($tag:expr) => {
		Component::Tag($tag.to_string())
	}
}
