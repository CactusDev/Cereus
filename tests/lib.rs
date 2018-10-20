
#[macro_use]
extern crate cereus;

use cereus::{
	command::manager::CommandManager,
	packet::*
};

fn get_example_text_only_context(packet: Packet) -> Context {
	Context {
		packet: packet,
		channel: "Stanley".to_string(),
		user: None,
		role: None,
		target: None,
		service: "Twitch".to_string()
	}
}

#[test]
fn test_command_name_only_resolves_to_default_handler() {
	let mut manager = CommandManager::new();
	manager.add_command(command!("cactus",
		"default" => handler!(|_context| {
			Packet::Message { text: vec! [
				text!("Hello!")
			], action: false }
		})
	));

	let context = get_example_text_only_context(Packet::Message {
		text: vec! [ text!("cactus") ],
		action: false
	});
	let resolved = manager.run_command(&context);
	assert!(resolved.is_some());

	let resolved = resolved.unwrap();
	// Ensure the packet contains the correct response
	match resolved {
		Packet::Message { text, action } => {
			assert_eq!(action, false);
			assert_eq!(text.len(), 1);
			match text[0] {
				Component::Text(ref text) => assert_eq!(text, "Hello!"),
				_ => assert!(false)
			}
		},
		_ => assert!(false)
	}
}

#[test]
fn test_command_name_with_single_valid_subcommand_argument_resolves_to_subcommands_handler() {
	let mut manager = CommandManager::new();
	manager.add_command(command!("cactus",
		"default" => handler!(|_context| {
			Packet::Message { text: vec! [
				text!("Hello!")
			], action: false }
		}),
		"test" => handler!(|_context| {
			Packet::Message { text: vec! [
				text!("Hello, world!")
			], action: false }
		})
	));

	let context = get_example_text_only_context(Packet::Message {
		text: vec! [ text!("cactus"), text!("test") ],
		action: false
	});
	let resolved = manager.run_command(&context);
	assert!(resolved.is_some());

	let resolved = resolved.unwrap();
	// Ensure the packet contains the correct response
	match resolved {
		Packet::Message { text, action } => {
			assert_eq!(action, false);
			assert_eq!(text.len(), 1);
			match text[0] {
				Component::Text(ref text) => assert_eq!(text, "Hello, world!"),
				_ => assert!(false)
			}
		},
		_ => assert!(false)
	}
}

#[test]
fn test_command_name_with_single_invalid_subcommand_argument_resolves_to_default_handler_and_passes_arguments() {
	let mut manager = CommandManager::new();
	manager.add_command(command!("cactus",
		"default" => handler!(|context| {
			Packet::Message { text: vec! [
				text!("This is a {}!", &if let Packet::Message { ref text, action } = context.packet {
					text[1].to_string()
				} else {
					"Unknown".to_string()
				})
			], action: false }
		})
	));

	let mut context = get_example_text_only_context(Packet::Message {
		text: vec! [ text!("cactus"), text!("test") ],
		action: false
	});
	let resolved = manager.run_command(&context);
	assert!(resolved.is_some());

	let resolved = resolved.unwrap();
	// Ensure the packet contains the correct response
	match resolved {
		Packet::Message { text, action } => {
			assert_eq!(action, false);
			assert_eq!(text.len(), 1);
			match text[0] {
				Component::Text(ref text) => assert_eq!(text, "This is a test!"),
				_ => assert!(false)
			}
		},
		_ => assert!(false)
	}
}
