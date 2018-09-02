#![allow(unused)]
#![feature(unsize, coerce_unsized, fn_traits, const_fn)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate log;

pub mod parse;
pub mod env;
pub mod obj;
pub mod shared;

pub use self::env::Environment;