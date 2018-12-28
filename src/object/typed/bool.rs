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
}