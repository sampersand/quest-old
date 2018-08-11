#![allow(unused)]
#![feature(never_type, try_trait)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate log;

#[macro_use]
mod macros;

mod sync;
mod env;
mod parse;
mod obj;
mod builtins;

pub use self::env::Environment;
pub use self::parse::{parse_str, parse_file};

