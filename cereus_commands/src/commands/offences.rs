
use crate::commands::Command;
use cereus_core::types::{Context, Component};

pub fn create_offences_command() -> Command {
    command!("offences",
        "modify" => handler!(|context, api, text, _action| {
            match text.as_slice() {
                [Component::Text(user), ty, modifier, _rest @ ..] => {
                    let modifier = modifier.to_string();
                    let (modifier, amount) = modifier.split_at(1);
                    if modifier != "+" && modifier != "-" && modifier != "@" || amount.len() < 1{
                        return Context::message(vec! [ text!("Invalid modifier specified!") ])
                    }

                    match api.update_user_offences(&context.channel, "TODO", user, &ty.to_string(), modifier, amount) {
                        Ok(()) => Context::message(vec! [ tag!(user), text!(" has been modified!") ]),
                        _ => Context::message(vec![ text!("Invalid offence type!") ])
                    }
                },
                _ => Context::message(vec! [ text!("Must provide a user!") ])
            }
        }),
        "get" => handler!(|context, api, text, _action| {
            match text.as_slice() {
                [Component::Text(user), ty, _rest @ ..] => {
                    match api.get_offences(&context.channel, "TODO", user, &ty.to_string()) {
                        Ok(offences) => Context::message(vec! [ tag!(user), text!(" {} offence count: {}", &ty.to_string(), &offences.to_string()) ]),
                        _ => Context::message(vec![ text!("Invalid offence type!") ])
                    }
                },
                _ => Context::message(vec! [ text!("Must provide a user!") ])
            }
        })
    )
}