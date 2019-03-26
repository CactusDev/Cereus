
extern crate cereus_commands;
#[macro_use]
extern crate cereus_core;

use cereus_core::types::{Context};
use std::vec::Vec;

pub trait Handler {
    fn run(&self, context: &Context) -> Vec<Option<Context>>;
}

pub struct HandlerHandler {
    handlers: Vec<Box<Handler>>
}

unsafe impl Sync for HandlerHandler {}
unsafe impl Send for HandlerHandler {}

impl HandlerHandler {

    pub fn new(handlers: Vec<Box<Handler>>) -> Self {
        HandlerHandler {
            handlers
        }
    }

    pub fn handle(&self, ctx: &Context) -> Vec<Context> {
        let mut contexts: Vec<Context> = Vec::new();

        'main: for handler in &self.handlers {
            for context in handler.run(ctx) {
                match context {
                    Some(context) => contexts.push(context.merge(ctx)),
                    None => {
                        // If we don't have anything here, that means this is a stop context, and we need
                        // to quit executing handlers, and return what we have now.
                        break 'main;
                    }
                }
            }
        }
        contexts
    }
}

pub mod command;
pub mod logging;
pub mod event;
pub mod spam;
