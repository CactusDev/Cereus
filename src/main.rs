
extern crate env_logger;
extern crate iron;
extern crate redis;

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
pub mod cache;
pub mod config;

#[derive(Serialize)]
struct Test {
    a: u32,
    b: String
}

impl cache::Cacheable for Test {

    fn name(&self) -> String {
        "stanley".to_string()
    }

    fn make_cacheable(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

fn main() {
    env_logger::init().unwrap();

    // TODO: Allow argument for configuration location.
    // Load the config
    let config = config::CereusConfiguration::new("cereus.json");
    match config {
        Ok(cfg) => {
            let mut cache = cache::Cache::new(30, "channel", &cfg.redis);
            let t = Test {
                a: 127,
                b: "Testing".to_string()
            };
            println!("{:?}", cache.cache(Box::new(t)));
            println!("{:?}", cache.get("stanley".to_string()))
        },
        Err(e) => {
            println!("Could not start Cereus due to a configuration error.");
            println!("{}", e);
        }
    }

    // let mut manager = command::manager::CommandManager::new();

    // manager.add_command(command!("cactus",
    // 	"default" => handler!(|_context| {
    // 		Context::message(vec! [
    // 			text!("Ohai! I'm CactusBot! "),
    // 			emoji!("cactus")
    // 		])
    // 	}),
    // 	"github" => handler!(
    // 		"default" => handler!(|_context| {
    // 			Context::message(vec! [
    // 				text!("We're open source! Check it out at: "),
    // 				url!("https://github.com/CactusDev")
    // 			])
    // 		}),
    // 		"cereus" => handler!(|_context| {
    // 			Context::message(vec! [
    // 				text!("Checkout Cereus at: "),
    // 				url!("https://github.com/CactusDev/Cereus")
    // 			])
    // 		})
    // 	)
    // ));

    // let logging_handler = handler::logging::LoggingHandler::new();
    // let event_handler = handler::event::EventHandler::new();
    // let command_handler = handler::command::CommandHandler::new("!", manager);
    // let spam_handler = handler::spam::SpamHandler::new();
    // let handler_handler = handler::HandlerHandler::new(vec! [
    //     Box::new(logging_handler), Box::new(spam_handler), Box::new(event_handler), Box::new(command_handler)
    // ]);

    // let w = web::WebServer::new("localhost", 1234, handler_handler);
    // w.listen();
}
