use lazy_static::lazy_static;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Boolean(bool);

impl Boolean {
	pub fn new(bool: bool) -> Boolean {
		Boolean(bool)
	}

	pub fn into_inner(self) -> bool {
		self.0
	}
}

impl Display for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Debug for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Boolean({:?})", self.0)
	}
}


impl_typed_conversion!(Boolean, bool);
impl_typed_object!(Boolean, new_bool, downcast_bool, is_bool);
impl_quest_conversion!("@bool" (as_bool_obj is_bool) (into_bool downcast_bool) -> Boolean);

impl_type! { for Boolean, downcast_fn=downcast_bool;
	fn "@text" (this) {
		this.0.to_string().into_object()
	}

	fn "@bool" (this) {
		this.into_object()
	}

	fn "@num" (this) {
		(this.0 as u8).into_object()
	}

	fn "==" (this, rhs) {
		(this == rhs.into_bool()?).into_object()
	}

	fn "!" (this) {
		(!this.0).into_object()
	}

	fn "xor" (this, rhs) {
		(this.0 != rhs.into_bool()?.0).into_object()
	}
}