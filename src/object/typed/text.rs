use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text(String);

impl Text {
	pub fn new(data: String) -> Text {
		Text(data)
	}

	pub fn into_inner(self) -> String {
		self.0
	}
}

impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Debug for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Text({:?})", self.0)
	}
}



impl_typed_conversion!(Text, String);
impl_typed_object!(Text, new_text, downcast_text, is_text);
impl_quest_conversion!("@text" (as_text_obj is_text) (into_text downcast_text) -> Text);

impl_type! { for Text, downcast_fn=downcast_text;
	fn "@var" (this) {
		Variable::from_string(this.0).into_object()
	}

	fn "@bool" (this) {
		(!this.0.is_empty()).into_object()
	}

	fn "@num" (this) { todo!() }

	fn "()" (this) { todo!("this will be a shell command"); }
	fn "eval" (this) { todo!("this will be evaluate, possibly with new env"); }

	fn "+" (this, rhs) {
		let mut this = this;
		this.0.push_str(&rhs.into_text()?.0);
		this.into_object()
	}

	fn "*" (this, rhs) {
		let lim = *rhs.into_num()?.into_integer().as_ref() as isize;
		if lim < 0 {
			return Ok("".to_string().into_object());
		}

		let mut new = String::with_capacity(this.0.len() * (lim as usize));
		for i in 0..lim {
			new.push_str(&this.0);
		}

		new.into_object()
	}

	fn "len" (this) {
		this.0.len().into_object()
	}

	fn "get" (this, index) { todo!() }
	fn "set" (this, index) { todo!() }
	fn "has" (this, index) { todo!() }
	fn "del" (this, index) { todo!() }


}

