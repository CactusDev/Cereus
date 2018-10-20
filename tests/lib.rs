
#[macro_use]
extern crate cereus;

use cereus::{
	command::manager::CommandManager,
	handler::{
		Handler,
		event::EventHandler
	},
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
				text!("This is a {}!", &if let Packet::Message { ref text, action: _ } = context.packet {
					text[1].to_string()
				} else {
					"Unknown".to_string()
				})
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
				Component::Text(ref text) => assert_eq!(text, "This is a test!"),
				_ => assert!(false)
			}
		},
		_ => assert!(false)
	}
}

#[test]
fn test_tri_subcommand_resolution() {
	let mut manager = CommandManager::new();
	manager.add_command(command!("cactus",
		"test" => handler! {
			"default" => handler!(|_context| {
				Packet::Message { text: vec! [
					text!("Hello!")
				], action: false }
			}),
			"another" => handler! {
				"default" => handler!(|_context| {
					Packet::Message { text: vec! [
						text!("Hello!")
					], action: false }
				}),
				"final" => handler! {
					"default" => handler!(|_context| {
						Packet::Message { text: vec! [
							text!("Hello!")
						], action: false }
					})
				}
			}
		}
	));

	let context = get_example_text_only_context(Packet::Message {
		text: vec! [ text!("!cactus"), text!("test"), text!("another"), text!("final") ],
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
fn test_start_with_new() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Start { new: true } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    assert!(result.is_some());
    match result.unwrap() {
    	Packet::Message { ref text, action } => {
    		assert_eq!(action, false);
    		assert_eq!(text.len(), 3);
    		assert_eq!(text[0], text!("Welcome to CactusBot. "));
    		assert_eq!(text[1], emoji!("cactus"));
    		assert_eq!(text[2], text!(" Type '!cactus help' for assistance."));
    	},
    	_ => assert!(false)
    }
}

#[test]
fn test_start_without_new() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Start { new: false } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    assert!(result.is_some());
    match result.unwrap() {
    	Packet::Message { ref text, action } => {
    		assert_eq!(action, false);
    		assert_eq!(text.len(), 2);
    		assert_eq!(text[0], text!("CactusBot activated. "));
    		assert_eq!(text[1], emoji!("cactus"));
    	},
    	_ => assert!(false)
    }
}

#[test]
fn test_follow_without_success() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Follow { success: false } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    assert!(result.is_none());
}


#[test]
fn test_follow_with_success() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Follow { success: true } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    assert!(result.is_some());
    match result.unwrap() {
    	Packet::Message { ref text, action } => {
    		assert_eq!(action, false);
    		assert_eq!(text.len(), 3);
    		assert_eq!(text[0], text!("Thanks for the follow, "));
    		assert_eq!(text[1], tag!("Stanley"));
    		assert_eq!(text[2], text!("!"));
    	},
    	_ => assert!(false)
    }
}
