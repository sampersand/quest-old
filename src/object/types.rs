#[macro_use]
mod macros;

mod basic;
mod number;
mod variable;
mod rustfn;
mod boolean;
mod text;
mod null;
mod map;
mod list;
mod oper;
mod block;
pub mod quest_funcs;

pub(super) mod pristine;

pub use self::{
	number::Number,
	rustfn::RustFn,
	variable::Variable,
	boolean::Boolean,
	text::Text,
	null::Null,
	map::Map,
	list::List,
	oper::Oper,
	block::Block
};

use crate::{shared::Shared, map::Map as MapTrait};
use std::hash::Hash;
use std::fmt::Debug;

pub trait Type : Debug + PartialEq + Hash + Send + Sync + Clone + 'static {
	fn get_type_map() -> Shared<dyn MapTrait>;
}

#[cfg(test)]
mod tests {
	use super::*;
	
	fn _is_type_send_sync<T: Type>() {
		fn _send_sync<T: Send + Sync>() {}
		_send_sync::<T>()
	}
}