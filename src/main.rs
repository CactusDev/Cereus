#![feature(slice_patterns)]

extern crate env_logger;
extern crate iron;
extern crate redis;
extern crate reqwest;
extern crate regex;
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod web;
#[macro_use]
pub mod packet;
pub mod handler;
#[macro_use]
pub mod command;
pub mod config;
pub mod types;

fn main() {
    env_logger::init().unwrap();

    // TODO: Allow argument for configuration location.
    // Load the config
    let config = config::CereusConfiguration::new("cereus.json");
    match config {
        Ok(cfg) => {
            let mut manager = command::manager::CommandManager::new("http://localhost:8000");

            manager.add_command(command::cactus::create_cactus_command());
            manager.add_command(command::command::create_command_command());

            let logging_handler = handler::logging::LoggingHandler::new();
            let event_handler = handler::event::EventHandler::new();
            let command_handler = handler::command::CommandHandler::new("!", manager);
            let spam_handler = handler::spam::SpamHandler::new();
            let handler_handler = handler::HandlerHandler::new(vec! [
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
