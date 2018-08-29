use obj::{SharedObject, types::IntoObject};
use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug, Display, Formatter};
use super::shared::{self, Offset};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Text(String);

impl From<String> for Text {
	#[inline]
	fn from(text: String) -> Text {
		Text(text)
	}
}

impl<'a> IntoObject for &'a str {
	type Type = Text;
	fn into_object(self) -> SharedObject<Text> {
		Text::from(self.to_string()).into_object()
	}
}

impl IntoObject for String {
	type Type = Text;
	fn into_object(self) -> SharedObject<Text> {
		Text::from(self).into_object()
	}
}

impl Deref for Text {
	type Target = String;

	#[inline]
	fn deref(&self) -> &String {
		&self.0
	}
}

impl DerefMut for Text {
	#[inline]
	fn deref_mut(&mut self) -> &mut String {
		&mut self.0
	}
}

impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl_type! {
	for Text, with self attr;

	fn "@text" (this) {
		Ok(this.duplicate())
	}

	fn "@bool" (this) {
		Ok((!this.data.is_empty()).into_object())
	}

	fn "+" (this, other) env, {
		let dup = this.duplicate();
		let d = dup.write();
		d.attrs.call("+=", &[&other.upgrade()], env)
	}

	fn "+=" (mut this, other) env, {
		this.data.push_str(&other.attrs.into_string(env)?);
		Ok(this.upgrade())
	}

	fn "[]" (this, start; end = Object::null()) env, {
		let start = start.attrs.into_num(env)?;
		let end = end.attrs.into_num(env).unwrap_or(start + Number::one());
		let len = this.data.len();

		let start = shared::offset(len, start)?;
		let end = shared::offset(len, end)?;

		use self::Offset::*;

		match (start, end) {
			(Valid(s), Valid(e)) if s < e => Ok(this[s..e].into_object()), // begin < end
			(Valid(_), Valid(_)) => Ok("".into_object()), // begin >= end

			(Valid(s), OutOfBounds(_)) => Ok(this[s..].into_object()), 
			_ => Ok(Object::null()) // everything else is nil
		}
	}

	fn _ (_) {
		any::get_default_attr(self, attr)
	}
}




