
#![feature(slice_concat_ext)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]

#[macro_use]
extern crate cereus_core;
extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod commands;
