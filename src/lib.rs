#![allow(unused)]
#![feature(coerce_unsized, unsize)]

#[macro_use]
extern crate log;

pub mod object;
pub mod shared;
pub mod env;
pub mod collections;
pub mod err;

use self::{
	shared::Shared,
	collections::Mapping,
	object::Object,
	env::Environment,
	err::{Error, Result}
};

pub type SharedObject = Object;