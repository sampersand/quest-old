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

use crate::{Shared, Environment, Object, Mapping};

use std::fmt::Debug;

pub trait Type : Eq + Debug + Clone + Send + Sync + 'static {
	fn create_map() -> Shared<dyn  Mapping>;
}

pub trait IntoObject : Sized {
	fn into_object(self) -> Object;
	fn into_shared(self) -> Shared<Object> {
		self.into_object().shared()
	}
}

impl<T: Into<Object>> IntoObject for T {
	fn into_object(self) -> Object {
		self.into()
	}
}

impl<T: Type + Sized> From<T> for Object {
	fn from(data: T) -> Object {
		Object::new(data)
	}
}