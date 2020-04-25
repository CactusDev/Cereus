
use crate:commands::Command;
use cereus_core::types::{Packet, Context, Component};

pub fn create_social_command() -> Command {
    command!("social",
        "default" => handler!(|context, api| {
            match context.packet {
                Packet::Message { ref text, action: _ } => {
                    let result = api.get_socials
                }
            }
        })
    )
}
