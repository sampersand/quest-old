use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;

impl From<Null> for Types {
	fn from(_: Null) -> Types {
		Types::Null
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
	
}





