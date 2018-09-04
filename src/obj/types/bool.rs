use parse::{Parsable, Stream};
use obj::{AnyShared, types::IntoObject};

impl_type! {
	for bool, with self attr;

	fn "@bool" (this) {
		Ok(this.read().duplicate())
	}

	fn "@num" (this) {
		Ok(Number::from(this.read().data as Integer).into_object())
	}

	fn _ () {
		any::get_default_attr(self, attr)
	}
}