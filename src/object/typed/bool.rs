use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bool(bool);

impl Bool {
	pub fn new(bool: bool) -> Bool {
		Bool(bool)
	}

	pub fn into_inner(self) -> bool {
		self.0
	}
}

impl Display for Bool {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl_typed_conversion!(Bool, bool);
impl_typed_object!(Bool, new_bool, downcast_bool, is_bool);
impl_quest_conversion!(as_bool -> Bool, "@bool" downcast_bool);

impl_type! { for Bool, downcast_fn=downcast_bool;
	fn "@text" (this) {
		this.0.to_string().into_object()
	}
}




