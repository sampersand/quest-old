#![allow(unused)]
#![cfg_attr(not(debug_assertions), deny(unreachable_code))]
#![feature(coerce_unsized, unsize, transpose_result)]

#[macro_use]
extern crate log;

#[macro_use]
mod macros;

pub mod parse;
mod object;
mod shared;
mod env;
mod collections;
mod err;

pub use self::{
	shared::Shared,
	collections::Mapping,
	object::{Object, IntoObject},
	env::Environment,
	err::{Error, Result}
};