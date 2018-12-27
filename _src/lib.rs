#![allow(unused)]
#![deny(unused_must_use)]
#![feature(unsize, coerce_unsized)]
// #![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate log;

extern crate rand;

// pub mod parse;
pub mod env;
pub mod obj;
pub mod shared;
pub mod err;

pub use self::env::Binding;