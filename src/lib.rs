#![allow(unused)]
#![feature(unsize, coerce_unsized, pattern, never_type)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate log;

#[macro_use]
mod macros;


mod shared;
mod env;
mod obj;
mod parse;


pub use self::env::Environment;
pub use self::parse::{parse_file};