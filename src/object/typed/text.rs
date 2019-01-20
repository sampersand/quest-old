use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text(String);

impl Text {
	pub fn new<T: Into<String>>(data: T) -> Text {
		Text(data.into())
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
	fn "@text" (this) {
		this.into_object()
	}

	fn "@var" (this) {
		super::Variable::from_string(this.0).into_object()
	}

	fn "@bool" (this) {
		(!this.0.is_empty()).into_object()
	}

	fn "@num" (this) {
		use crate::parse::{ParseFromStr, ParseOk};
		match super::Number::from_str(&this.0) {
			Ok(ParseOk::NotFound) => Object::new_null(),
			Ok(ParseOk::Found(num, _)) => num.into_object(),
			Err(err) => return Err(crate::Error::Boxed(Box::new(err)))
		}
	}

	fn "==" (this, rhs) {
		(this == rhs.into_text()?).into_object()
	}

	fn "()" (_this) { todo!("this will be a shell command"); }
	fn "eval" (_this) { todo!("this will be evaluate, possibly with new env"); }

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
		for _ in 0..lim {
			new.push_str(&this.0);
		}

		new.into_object()
	}

	fn "len" (this) {
		this.0.len().into_object()
	}

	fn "get" (_this, _index) { todo!() }
	fn "set" (_this, _index, _val) { todo!() }
	fn "has" (_this, _index) { todo!() }
	fn "del" (_this, _index) { todo!() }
}

#[cfg(test)]
mod rust_tests {
	use super::Text;
	use crate::object::IntoObject;

	#[test]
	fn to_string_conversion_works(){
		assert_eq!(String::from("where is the sun"), "where is the sun".to_string());
	}

	#[test]
	fn empty_string() {
		assert_eq!(String::from(Text::new("")), "".to_string());
	}

	#[test]
	fn preserves_contents() {
		assert_eq!(String::from(Text::new("my contents")), "my contents".to_string());
	}

	#[test]
	fn create_string_object() {
		assert_eq!("hi friend".to_string().into_object().downcast_text().unwrap(), Text::new("hi friend"));
	}

	#[test]
	fn is_text() {
		assert!("lol".to_string().into_object().is_text());
	}

	#[test]
	fn object_eql_works() {
		assert_eq!("hi there".to_string().into_object(), "hi there".to_string().into_object());
	}
}


#[cfg(test)]
#[allow(unused)]
mod quest_tests {
	use super::Text;
	use crate::object::{IntoObject};

	macro_rules! text {
		($x:expr) => ( Text::new($x) );
	}


	#[test]
	fn at_text() {
		// assert_eq!(make!(text "fooey").into_text().unwrap(), text!("fooey"));
	}

	#[test]
	fn at_var() {
		// assert_eq!(tobj!("fooey").into_var().unwrap(), text!("fooey"));
	}


// impl_type! { for Text, downcast_fn=downcast_text;
// 	fn "@text" (this) {
// 		this.into_object()
// 	}

// 	fn "@var" (this) {
// 		super::var::Variable::from_string(this.0).into_object()
// 	}

// 	fn "@bool" (this) {
// 		(!this.0.is_empty()).into_object()
// 	}

// 	fn "@num" (_this) { todo!() }

// 	fn "==" (this, rhs) {
// 		(this == rhs.into_text()?).into_object()
// 	}

// 	fn "()" (_this) { todo!("this will be a shell command"); }
// 	fn "eval" (_this) { todo!("this will be evaluate, possibly with new env"); }

// 	fn "+" (this, rhs) {
// 		let mut this = this;
// 		this.0.push_str(&rhs.into_text()?.0);
// 		this.into_object()
// 	}

// 	fn "*" (this, rhs) {
// 		let lim = *rhs.into_num()?.into_integer().as_ref() as isize;
// 		if lim < 0 {
// 			return Ok("".to_string().into_object());
// 		}

// 		let mut new = String::with_capacity(this.0.len() * (lim as usize));
// 		for _ in 0..lim {
// 			new.push_str(&this.0);
// 		}

// 		new.into_object()
// 	}

// 	fn "len" (this) {
// 		this.0.len().into_object()
// 	}

// 	fn "get" (_this, _index) { todo!() }
// 	fn "set" (_this, _index, _val) { todo!() }
// 	fn "has" (_this, _index) { todo!() }
// 	fn "del" (_this, _index) { todo!() }
// }
}






















