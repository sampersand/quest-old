#![allow(unused)]
#![deny(unused_must_use)]
#![feature(unsize, coerce_unsized, fn_traits, const_fn, fnbox, transpose_result)]
#![recursion_limit = "1024"]
#![allow(deprecated)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate log;

extern crate rand;

pub mod parse;
pub mod env;
pub mod obj;
pub mod shared;

pub use self::env::Environment;