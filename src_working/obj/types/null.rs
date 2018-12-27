use std::fmt::{self, Display, Formatter};
use obj::{AnyObject, Object, AnyShared};

impl AnyObject {
	pub fn null() -> AnyShared {
		Object::new(Null) as AnyShared
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Display for Null {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt("null", f)
	}
}

__impl_type! {
	for Null, with self attr;

	fn "@bool" (_) {
		Ok(false.into_object())
	}

	fn "()" (_) {
		Ok(Object::null())
	}

	fn _ () {
		any::__get_default(self, attr)
	}
}