
#[macro_use]
use command::{Command};
use packet::Context;

pub fn quote_command() -> Command {
    command!("quote",
        "default" => handler!(|_context, _api| {
            // Get a random quote
            Context::message(vec! [])
        })
    )
}
