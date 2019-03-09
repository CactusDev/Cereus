
#[macro_use]
extern crate cereus;

use cereus::{
	command::manager::CommandManager,
	handler::{
		Handler,
		event::EventHandler,
        spam::SpamHandler
	},
	packet::*
};

fn get_example_text_only_context(packet: Packet) -> Context {
	Context {
		packet: packet,
		channel: "Stanley".to_string(),
		user: Some("Stanley".to_string()),
		role: None,
		target: None,
		service: Some("Twitch".to_string()),
        count: None
	}
}

#[test]
fn test_command_name_only_resolves_to_default_handler() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
	manager.add_command(command!("cactus",
		"default" => handler!(|_context, _api| {
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
	let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
	manager.add_command(command!("cactus",
		"default" => handler!(|_context, _api| {
			Context::message(vec! [
				text!("Hello!")
			])
		}),
		"test" => handler!(|_context, _api| {
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
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
	manager.add_command(command!("cactus",
		"default" => handler!(|context, _api| {
			Context::message(vec! [
				text!("This is a "),
                text!("{}!", &if let Packet::Message { ref text, action: _ } = context.packet {
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
        text!("This is a "),
        text!("test!")
    ], action: false };

    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_tri_subcommand_resolution() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
	manager.add_command(command!("cactus",
		"test" => handler! {
			"default" => handler!(|_context, _api| {
                Context::message(vec! [
					text!("Hello!")
                ])
			}),
			"another" => handler! {
				"default" => handler!(|_context, _api| {
					Context::message(vec! [
						text!("Hello!")
					])
				}),
				"final" => handler! {
					"default" => handler!(|_context, _api| {
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
fn test_command_user_formatter() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%USER%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("Stanley!"),
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_args_formatter() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARGS%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("test"), text!("ing") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("test ing!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("test") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("test!"),
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_out_of_range() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG2%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("test") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_additional_dangling_arguments() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("test"), text!("other") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("test!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_args_formatter_with_default_arguments_and_none_provided() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARGS=user%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("user!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_args_formatter_with_default_arguments_with_provided() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARGS=user%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("test") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("test!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}



#[test]
fn test_command_argn_formatter_with_default_arguments_and_none_provided() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1=user%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("user!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_default_arguments_with_provided() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1=user%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("test") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("test!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_default_arguments_with_provided_modifier_upper() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1=user|upper%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("test") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("TEST!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_default_arguments_with_none_modifier_upper() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1=user|upper%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("USER!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_modifier_lower() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1|lower%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("TEST") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("test!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_modifier_title() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1|title%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("TEST") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("Test!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_modifier_reverse() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1|reverse%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("thing") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("gniht!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_modifier_tag() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("@%ARG1|tag%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("@stanley") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("@stanley!")
    ], action: false };
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_modifier_shuffle() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1|shuffle%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("tearstarstarstarstarstarstarstarstarstarstarstarstarstst") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("tearstarstarstarstarstarstarstarstarstarstarstarstarstst!")
    ], action: false };
    assert!(resolved.is_some());
    assert_ne!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_argn_formatter_with_modifier_chain() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("Hello "),
                text!("%ARG1|title|tag%!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd"), text!("@test") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("Hello "),
        text!("@Test!")
    ], action: false };
    assert!(resolved.is_some());
    assert_ne!(resolved.unwrap().packet, first_packet);
}

#[test]
fn test_command_channel_formatter() {
    let mut manager = CommandManager::new("https://api.cactus.opsywopsy.science/v1");
    manager.add_command(command!("cmd",
        "default" => handler!(|_context, _api| {
            Context::message(vec! [
                text!("This is the "),
                text!("%CHANNEL% "),
                text!("channel!")
            ])
        })
    ));

    let context = get_example_text_only_context(Packet::Message {
        text: vec! [ text!("cmd") ],
        action: false
    });

    let resolved = manager.run_command(&context);

    let first_packet = Packet::Message { text: vec! [
        text!("This is the "),
        text!("Stanley "),
        text!("channel!")
    ], action: false };
    assert!(resolved.is_some());
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
        service: Some("Twitch".to_string()),
        count: None
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
	
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };

    match result[1] {
        Some(ref r) => assert_eq!(r.packet, second_packet),
        None => assert!(false)
    };
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
        service: Some("Twitch".to_string()),
        count: None
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("CactusBot activated. "),
        emoji!("cactus")
    ], action: false };

    assert_eq!(result.len(), 1);
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };
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
        service: Some("Twitch".to_string()),
        count: None
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
        service: Some("Twitch".to_string()),
        count: None
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for the follow, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };
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
        service: Some("Twitch".to_string()),
        count: None
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for subscribing, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };
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
        service: Some("Twitch".to_string()),
        count: None
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for resubscribing for 2 months, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };
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
        service: Some("Twitch".to_string()),
        count: None
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
        service: Some("Twitch".to_string()),
        count: None
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for the host, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };
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
        service: Some("Twitch".to_string()),
        count: None
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Welcome, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    }
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
        service: Some("Twitch".to_string()),
        count: None
    };
    let result = handler.run(&context);
    let first_packet = Packet::Message { text: vec! [
        text!("Thanks for watching, "),
        tag!("Stanley"),
        text!("!")
    ], action: false };

    assert_eq!(result.len(), 1);
    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };
}

#[test]
fn test_spam_compliant_message() {
    let context = Context {
        packet: Packet::Message {
            text: vec! [
                text!("This is a test!")
            ], action: false
        },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: Some("Twitch".to_string()),
        count: None     
    };
    let handler = SpamHandler::new();
    let result = handler.run(&context);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_spam_caps_message() {
    let context = Context {
        packet: Packet::Message {
            text: vec! [
                text!("WOW THESE ARE A LOT OF CAPS THAT SHOULD MAKE ANGRY"),
            ], action: false
        },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: Some("Twitch".to_string()),
        count: None
    };
    let handler = SpamHandler::new();
    let result = handler.run(&context);

    assert_eq!(result.len(), 3);

    let first_packet = Packet::Message { text: vec! [
        text!("Please do not spam capital letters.")
    ], action: false };

    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    }

    match result[1] {
        Some(ref r) => match r.packet {
            Packet::Ban { duration } => assert_eq!(duration, None),
            _ => assert!(false)
        },
        None => assert!(false)
    }
}

#[test]
fn test_spam_emoji_message() {
    let context = Context {
        packet: Packet::Message {
            text: vec! [
                emoji!("cactus"),
                emoji!("cactus"),
                emoji!("cactus"),
                emoji!("cactus"),
                emoji!("cactus"),
                emoji!("cactus"),
                emoji!("cactus")
            ], action: false
        },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: Some("Twitch".to_string()),
        count: None     
    };
    let handler = SpamHandler::new();
    let result = handler.run(&context);

    assert_eq!(result.len(), 3);

    let first_packet = Packet::Message { text: vec! [
        text!("Please do not spam emoji.")
    ], action: false };

    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };

    match result[1] {
        Some(ref r) => match r.packet {
            Packet::Ban { duration } => assert_eq!(duration, None),
            _ => assert!(false)
        },
        None => assert!(false)
    }
}

#[test]
fn test_spam_url_message() {
    let context = Context {
        packet: Packet::Message {
            text: vec! [
                url!("google.com")
            ], action: false
        },
        channel: "".to_string(),
        user: Some("Stanley".to_string()),
        role: None,
        target: None,
        service: Some("Twitch".to_string()),
        count: None 
    };
    let handler = SpamHandler::new();
    let result = handler.run(&context);

    assert_eq!(result.len(), 3);

    let first_packet = Packet::Message { text: vec! [
        text!("Please do not post URLs.")
    ], action: false };

    match result[0] {
        Some(ref r) => assert_eq!(r.packet, first_packet),
        None => assert!(false)
    };

    match result[1] {
        Some(ref r) => match r.packet {
            Packet::Ban { duration } => assert_eq!(duration, None),
            _ => assert!(false)
        },
        None => assert!(false)
    };
}
