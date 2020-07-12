
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    pub static ref ARGN_REGEX: Regex = Regex::new(r#"%ARG(\d+)(?:=([^|]+))?(?:((?:\|\w+)+))?%"#).unwrap();
    pub static ref ARGS_REGEX: Regex = Regex::new(r#"%ARGS(?:=([^|]+))?((?:\|\w+)+)?%"#).unwrap();
    pub static ref COMMAND_PREFIX: Vec<String> = vec! [ "+".to_string(), "$".to_string() ];
}

pub mod types;
