
use crate::commands::Command;
use cereus_core::types::{Context, Component, Trust};

fn make_trust_string(trusts: Vec<Trust>) -> Option<String> {
    if trusts.len() == 0 {
        return None
    }

    return Some(trusts.iter().fold(String::new(),
                  |a, b| if a.len() > 0 { a + ", " } else { a } + &b.trusted));
}

pub fn create_trust_command() -> Command {
    command!("trust",
        "list" => handler!(|context, api, _text, _action| {
            let result = api.get_trusts(&context.channel);
            match result {
                Ok(trusts) => match make_trust_string(trusts) {
                    Some(trusts) => Context::message(vec! [
                        text!("Trusted users: "),
                        text!(trusts)
                    ]),
                    None => Context::message(vec! [ text!("No users are trusted!") ])
                },
                Err(_) => Context::message(vec! [ text!("No users are trusted!") ])
            }
        }),
        "add" => handler!(|context, api, text, _action| {
            match text.as_slice() {
                [Component::Text(user), _rest @ ..] => match api.add_trust(&context.channel, &user) {
                    Ok(()) => Context::message(vec! [ tag!(user), text!(" is now trusted!") ]),
                    _ => Context::message(vec![ tag!(user), text!(" was already trusted.") ])
                },
                _ => Context::message(vec! [ text!("Must provide a user!") ])
            }
        }),
        "remove" => handler!(|context, api, text, _action| {
            match text.as_slice() {
                [Component::Text(user), _rest @ ..] => match api.remove_trust(&context.channel, &user) {
                    Ok(()) => Context::message(vec! [ text!(user), text!(" is no longer trusted!") ]),
                    _ => Context::message(vec![ tag!(user), text!(" is not trusted.") ])
                },
                _ => Context::message(vec! [ text!("Must provide a user!") ])
            }
        })
    )
}
