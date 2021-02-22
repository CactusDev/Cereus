
use crate::commands::Command;
use cereus_core::types::Context;

pub fn create_alias_command() -> Command {
    command!("alias",
        "default" => handler!(|_context, _api, _text, _action| {
            Context::message(vec! [
                text!("Ohai! I'm CactusBot! "),
                emoji!("cactus")
            ])
        })
    )
}
