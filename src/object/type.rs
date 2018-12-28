mod number;
mod variable;
mod map;
mod list;

pub use self::{
	number::Number,
	variable::Variable,
	map::Map,
	list::List,
};

use crate::{Object};

use std::fmt::Debug;

pub trait Type : Eq + Debug + Clone + Send + Sync + 'static {
	fn create_map() -> Object;
}

pub trait IntoObject : Sized {
	fn into_object(self) -> Object;
}