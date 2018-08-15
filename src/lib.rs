#![allow(unused)]
#![feature(unsize, coerce_unsized, pattern, never_type, rc_downcast)]
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

pub use self::env::Binding;