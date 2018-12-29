use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bool(bool);

impl Type for Bool {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new({
				unimplemented!();
			});
		}
		Shared::new(ParentalMap::new_default(PARENT.clone()))
	}
}

impl From<Bool> for bool {
	fn from(bool: Bool) -> bool {
		bool.0
	}
}

impl From<bool> for Bool {
	fn from(bool: bool) -> Bool {
		Bool(bool)
	}
}

impl From<Bool> for Types {
	fn from(bool: Bool) -> Types {
		Types::Bool(bool)
	}
}



impl TypedObject {
	pub fn new_bool<T: Into<Bool>>(val: T) -> Self {
		TypedObject::new(val.into())
	}

	pub fn downcast_bool(&self) -> Option<&Bool> {
		if let Types::Bool(ref bool) = self.data {
			Some(bool)
		} else {
			None
		}
	}

}

impl Shared<Object> {
	/// note: this clones the object
	pub fn downcast_bool(&self) -> Option<Bool> {
		self.read().map.read()
		    .downcast_ref::<TypedObject>()
		    .and_then(TypedObject::downcast_bool)
		    .cloned()
	}

	pub fn into_bool(self) -> Option<bool> {
		self.downcast_bool().map(Into::into)
	}
}