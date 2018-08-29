impl_type! {
	for bool, with self attr;

	fn "@bool" (this) {
		Ok(this.duplicate())
	}

	fn "@num" (this) {
		Ok(Number::from_integer(this.data as _).into_object())
	}

	fn _ (_) {
		any::get_default_attr(self, attr)
	}
}