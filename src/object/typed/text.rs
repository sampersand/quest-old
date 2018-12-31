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
impl_quest_conversion!("@text" (as_text_obj is_text) (as_text downcast_text) -> Text);

impl_type! { for Text, downcast_fn=downcast_text;
	fn "@var" (this) {
		Var::from_string(this.0).into_object()
	}
}








