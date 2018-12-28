#![allow(unused)]

mod object;
mod shared;
mod collections;
mod env;

use self::{
	shared::Shared,
	collections::{Map, List},
	object::Object,
	env::Environment
};