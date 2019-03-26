
#![feature(slice_patterns)]
#![feature(slice_concat_ext)]

#[macro_use]
extern crate cereus_core;
extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod commands;
