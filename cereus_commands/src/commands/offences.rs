
use crate::commands::Command;
use cereus_core::types::{Context, Component};

pub fn create_offences_command() -> Command {
    command!("offences",
        "modify" => handler!(|context, api, text, _action| {
            match text.as_slice() {
                [Component::Text(user), Component::Text(ty), Component::Text(modifier), _rest @ ..] => {
                    let (modifier, amount) = modifier.split_at(1);
                    if modifier != "+" && modifier != "-" && modifier != "=" || amount.len() < 1{
                        return Context::message(vec! [ text!("Invalid modifier specified!") ])
                    }

                    match api.update_user_offences(&context.channel, "TODO", user, &ty, modifier, amount) {
                        Ok(()) => Context::message(vec! [ tag!(user), text!(" has been modified!") ]),
                        _ => Context::message(vec![ text!("Invalid offence type!") ])
                    }
                },
                _ => Context::message(vec! [ text!("Must provide a user!") ])
            }
        }),
        "get" => handler!(|context, api, text, _action| {
            match text.as_slice() {
                [Component::Text(user), Component::Text(ty), _rest @ ..] => {
                    match api.get_offences(&context.channel, "TODO", user, &ty) {
                        Ok(offences) => Context::message(vec! [ tag!(user), text!(" {} offence count: {}", &ty.to_string(), &offences.to_string()) ]),
                        _ => Context::message(vec![ text!("Invalid offence type!") ])
                    }
                },
                _ => Context::message(vec! [ text!("Must provide a user!") ])
            }
        })
    )
}
