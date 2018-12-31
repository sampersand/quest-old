use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use std::fmt::{self, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;

impl From<Null> for Types {
	fn from(_: Null) -> Types {
		Types::Null
	}
}

impl Display for Null {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "null")
	}
}


impl TypedObject {
	pub fn new_null() -> Self {
		TypedObject::new(Null)
	}

	pub fn is_null(&self) -> bool {
		self.data == Types::Null
	}

}

impl Object {
	pub fn new_null() -> Self {
		Object::new(TypedObject::new_null())
	}

	/// note: this clones the object
	pub fn is_null(&self) -> bool {
		self.map().read()
		    .downcast_ref::<TypedObject>()
		    .map(TypedObject::is_null)
		    .unwrap_or(false)
	}

	pub fn downcast_null(&self) -> Option<Null> {
		if self.is_null() {
			Some(Null)
		} else {
			None
		}
	}
}



impl_type! { for Null, downcast_fn=downcast_null;
	fn "@text" (_) {
		"null".to_string().into_object()
	}

	fn "@bool" (_) {
		false.into_object()
	}
}





