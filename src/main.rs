
extern crate iron;

extern crate cereus_core;
extern crate cereus_handlers;
extern crate cereus_commands;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use cereus_handlers::{HandlerHandler, command::CommandHandler, event::EventHandler, logging::LoggingHandler, spam::SpamHandler};
use cereus_commands::commands::{manager::CommandManager, cactus::create_cactus_command, command::create_command_command, quote::create_quote_command, multi::create_multi_command, trust::create_trust_command, social::create_social_command, offences::create_offences_command};

mod web;
mod config;

fn main() {
    // TODO: Allow argument for configuration location.
    // Load the config
    let config = config::CereusConfiguration::new("cereus.json");
    match config {
        Ok(cfg) => {
            let mut manager = CommandManager::new("http://localhost:8000");

            manager.add_command(create_cactus_command());
            manager.add_command(create_command_command());
            manager.add_command(create_quote_command());
            manager.add_command(create_multi_command());
            manager.add_command(create_trust_command());
            manager.add_command(create_social_command());
            manager.add_command(create_offences_command());

            let logging_handler = LoggingHandler::new();
            let event_handler = EventHandler::new();
            let command_handler = CommandHandler::new("!", manager);
            let spam_handler = SpamHandler::new();
            let handler_handler = HandlerHandler::new(vec! [
                Box::new(logging_handler), Box::new(spam_handler),
                Box::new(event_handler),   Box::new(command_handler)
            ]);

            let w = web::WebServer::new("127.0.0.1", cfg.port, handler_handler);
            w.listen();
        },
        Err(e) => {
            println!("Could not start Cereus due to a configuration error.");
            println!("{}", e);
        }
    }
}
