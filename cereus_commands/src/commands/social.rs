
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
                if i != services.len() {
                    result.push(text!(","));
                }
            }
            return Context::message(result)
        })
    )
}
