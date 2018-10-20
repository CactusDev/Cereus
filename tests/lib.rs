
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
			Context::message(vec! [
				text!("Hello!")
			])
		})
	));

	let context = get_example_text_only_context(Packet::Message {
		text: vec! [ text!("cactus") ],
		action: false
	});
	let resolved = manager.run_command(&context);
	assert!(resolved.is_some());

    let first_packet = Packet::Message { text: vec! [
        text!("Hello!"),
    ], action: false };
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_name_with_single_valid_subcommand_argument_resolves_to_subcommands_handler() {
	let mut manager = CommandManager::new();
	manager.add_command(command!("cactus",
		"default" => handler!(|_context| {
			Context::message(vec! [
				text!("Hello!")
			])
		}),
		"test" => handler!(|_context| {
			Context::message(vec! [
				text!("Hello, world!")
			])
		})
	));

	let context = get_example_text_only_context(Packet::Message {
		text: vec! [ text!("cactus"), text!("test") ],
		action: false
	});

	let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello, world!"),
    ], action: false };
	assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_name_with_single_invalid_subcommand_argument_resolves_to_default_handler_and_passes_arguments() {
	let mut manager = CommandManager::new();
	manager.add_command(command!("cactus",
		"default" => handler!(|context| {
			Context::message(vec! [
				text!("This is a {}!", &if let Packet::Message { ref text, action: _ } = context.packet {
					text[1].to_string()
				} else {
					"Unknown".to_string()
				})
			])
		})
	));

	let context = get_example_text_only_context(Packet::Message {
		text: vec! [ text!("cactus"), text!("test") ],
		action: false
	});
	let resolved = manager.run_command(&context);
	assert!(resolved.is_some());

    let first_packet = Packet::Message { text: vec! [
        text!("This is a test!")
    ], action: false };

    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_tri_subcommand_resolution() {
	let mut manager = CommandManager::new();
	manager.add_command(command!("cactus",
		"test" => handler! {
			"default" => handler!(|_context| {
                Context::message(vec! [
					text!("Hello!")
                ])
			}),
			"another" => handler! {
				"default" => handler!(|_context| {
					Context::message(vec! [
						text!("Hello!")
					])
				}),
				"final" => handler! {
					"default" => handler!(|_context| {
                        Context::message(vec! [
							text!("Hello!")
						])
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

    let first_packet = Packet::Message { text: vec! [
        text!("Hello!")
    ], action: false };
    assert_eq!(resolved.unwrap().packet, first_packet);
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
    assert_eq!(result.len(), 2);

    let first_packet = Packet::Message { text: vec! [
        text!("Welcome to CactusBot. "),
        emoji!("cactus")
    ], action: false };

    let second_packet = Packet::Message { text: vec! [
        text!("Type '!cactus help' for assistance."),
    ], action: false };
	
	assert_eq!(result[0].packet, first_packet);
	assert_eq!(result[1].packet, second_packet);
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
    let first_packet = Packet::Message { text: vec! [
        text!("CactusBot activated. "),
        emoji!("cactus")
    ], action: false };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].packet, first_packet);
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
    assert_eq!(result.len(), 0);
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
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for the follow, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].packet, first_packet);
}

#[test]
fn test_subscribe_with_streak_one() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Subscribe { streak: 1 } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for subscribing, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].packet, first_packet);
}

#[test]
fn test_subscribe_with_different_streak() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Subscribe { streak: 2 } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for resubscribing for 2 months, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].packet, first_packet);
}

#[test]
fn test_host_without_success() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Host { success: false } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_host_with_success() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Host { success: true } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for the host, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].packet, first_packet);
}

#[test]
fn test_join() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Join { success: true } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Welcome, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].packet, first_packet);
}

#[test]
fn test_leave() {
	let handler = EventHandler::new();

	let context = Context {
        packet: Packet::Event { kind: Event::Join { success: false } },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: "".to_string()
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for watching, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].packet, first_packet);
}
