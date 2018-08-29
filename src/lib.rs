#![allow(unused)]
#![feature(unsize, coerce_unsized, fn_traits, never_type)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate log;

mod parse;
mod env;
mod obj;
mod shared;

pub use self::env::Environment;
pub use self::obj::_foo;