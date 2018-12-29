use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bool(bool);

impl Type for Bool {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Object = Shared::new({
				unimplemented!();
			});
		}
		Shared::new(ParentalMap::new_default(|| PARENT.clone()))
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

impl_typed_object!(Bool, new_bool, downcast_bool);

impl Object {
	pub fn into_bool(self) -> Option<bool> {
		self.downcast_bool().map(Into::into)
	}
}