#[macro_use]
mod macros;

mod basic;
mod number;
mod variable;
mod rustfn;
mod boolean;
mod text;

pub(super) mod pristine;

pub use self::{
	number::Number,
	rustfn::RustFn,
	variable::Variable,
	boolean::Boolean,
	text::Text
};

use crate::{shared::Shared, map::Map};
use std::hash::Hash;
use std::fmt::Debug;

pub trait Type : Debug + PartialEq + Hash + Send + Sync + 'static {
	fn get_type_map() -> Shared<dyn Map>;
}

#[cfg(test)]
mod tests {
	use super::*;
	
	fn _is_type_send_sync<T: Type>() {
		fn _send_sync<T: Send + Sync>() {}
		_send_sync::<T>()
	}
}