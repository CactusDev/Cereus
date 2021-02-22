
use crate::commands::Command;
use cereus_core::types::{Context, Component, Social};

pub fn create_social_command() -> Command {
    command!("social",
        "default" => handler!(|context, api, text, _action| {
            let mut services: Vec<Social> = Vec::new();
            if text.len() > 0 {
                let service = text[0].to_string();
                match api.get_social(&context.channel, &service) {
                    Ok(s) => services.push(s),
                    Err(_) => return Context::message(vec! [ text!("No social platform added with that named!") ])
                }
            } else {
                match api.get_socials(&context.channel) {
                    Ok(s) => services = s,
                    Err(_) => return Context::message(vec! [ text!("No social platforms added!") ])
                }
            }

            let mut result: Vec<Component> = Vec::new();
            for (i, service) in services.iter().enumerate() {
                result.push(text!(format!("{}: {}", service.service, service.url)));
                if i < (services.len() - 1) {
                    result.push(text!(","));
                }
            }
            return Context::message(result);
        }),
        "add" => handler!(|context, api, text, _action| {
            if let Some((service, address)) = text.split_first() {
                if address.len() < 1 {
                    return Context::message(vec! [ text!("Must provide a service!") ]);
                }

                if let Component::Text(service) = service {
                    let address = &address[0].to_string();

                    return match api.add_social(&context.channel, &service, &address) {
                        Ok(()) => Context::message(vec! [ text!("Social service "), text!(service), text!(" has been added!") ]),
                        Err(_) => Context::message(vec! [ text!("Unable to add service!") ])
                    };

                }
            }
            return Context::message(vec! [ text!("Invalid syntax! !social add <service> <address>") ]);
        }),
        "remove" => handler!(|context, api, text, _action| {
            if let Some((service, _)) = text.split_first() {
                if let Component::Text(service) = service {
                    return match api.remove_social(&context.channel, &service) {
                        Ok(()) => Context::message(vec! [ text!("Social service "), text!(service), text!(" has been removed!") ]),
                        Err(_) => Context::message(vec! [ text!("Unable to remove service!") ])
                    };
                }
            }
            return Context::message(vec! [ text!("Invalid syntax! !social remove <service> <address>") ]);
        })
    )
}
