use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

impl Debug for Bool {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Bool({:?})", self.0)
	}
}


impl_typed_conversion!(Bool, bool);
impl_typed_object!(Bool, new_bool, downcast_bool, is_bool);
impl_quest_conversion!("@bool" (as_bool_obj is_bool) (into_bool downcast_bool) -> Bool);

impl_type! { for Bool, downcast_fn=downcast_bool;
	fn "@text" (this) {
		this.0.to_string().into_object()
	}

	fn "@bool" (this) {
		this.into_object()
	}

	fn "@num" (this) {
		(this.0 as u8).into_object()
	}

	fn "!" (this) {
		(!this.0).into_object()
	}

	fn "xor" (this, rhs) {
		(this.0 != rhs.into_bool()?.0).into_object()
	}
}







