// #![allow(unused)]
#![cfg_attr(not(debug_assertions), deny(unreachable_code))]
#![feature(coerce_unsized, unsize)]

#[macro_use]
extern crate log;

#[macro_use]
mod macros;

// mod parse;
mod object;
mod shared;
mod env;
mod collections;
mod err;

pub use self::env::builtins::BUILTINS_MAP as __BUILTINS_MAP;

pub use self::{
	shared::Shared,
	collections::Mapping,
	object::{Object, IntoObject},
	env::Environment,
	err::{Error, Result}
};