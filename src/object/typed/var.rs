use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var(&'static str);

impl Type for Var {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new({
				Object::new(crate::collections::Map::default())
			});
		}
		Shared::new(ParentalMap::new_default(PARENT.clone()))
	}
}

impl From<&'static str> for Var {
	fn from(id: &'static str) -> Var {
		Var(id)
	}
}

impl From<Var> for Types {
	fn from(id: Var) -> Types {
		Types::Var(id)
	}
}


impl TypedObject {
	pub fn new_var<T: Into<Var>>(val: T) -> Self {
		TypedObject::new(val.into())
	}

	pub fn downcast_var(&self) -> Option<&Var> {
		if let Types::Var(ref var) = self.data {
			Some(var)
		} else {
			None
		}
	}

}

impl Shared<Object> {
	/// note: this clones the object
	pub fn downcast_var(&self) -> Option<Var> {
		self.read().map.read()
		    .downcast_ref::<TypedObject>()
		    .and_then(TypedObject::downcast_var)
		    .cloned()
	}
}