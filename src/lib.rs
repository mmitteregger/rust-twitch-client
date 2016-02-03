#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use] extern crate hyper;
extern crate serde;
extern crate serde_json;

pub mod model;
pub mod error;
pub mod client;

