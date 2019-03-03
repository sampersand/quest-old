use std::fmt::{self, Display, Formatter};
use crate::object::{Object, AnyObject};
use crate::err::Result;
use std::ops::Deref;
use super::quest_funcs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Boolean(bool);

impl Boolean {
	#[inline]
	pub fn new(boolean: bool) -> Boolean {
		Boolean(boolean)
	}

	pub fn is_true(&self) -> bool {
		self.0 == true
	}
}

impl Object<Boolean> {
	pub fn new_boolean(boolean: bool) -> Object<Boolean> {
		Object::new(Boolean::new(boolean))
	}

	pub fn is_true(&self) -> bool {
		self.data().read().expect("read error in Object::is_true").is_true()
	}
}

impl AnyObject {
	pub fn to_boolean(&self) -> Result<Object<Boolean>> {
		self.call_attr(quest_funcs::AT_BOOL, &[])?
			.downcast_or_err::<Boolean>()
	}
}

impl From<bool> for Boolean {
	fn from(boolean: bool) -> Boolean {
		Boolean::new(boolean)
	}
}

impl From<Boolean> for bool {
	fn from(boolean: Boolean) -> bool {
		boolean.0
	}
}

impl Deref for Boolean {
	type Target = bool;
	fn deref(&self) -> &bool {
		&self.0
	}
}

impl AsRef<bool> for Boolean {
	fn as_ref(&self) -> &bool {
		&self.0
	}
}

impl Display for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

mod funcs {
	use crate::object::types::{Number, Boolean, Text};
	use crate::object::Object;

	pub const TRUE_STR: &str = "true";
	pub const FALSE_STR: &str = "false";
	pub const TRUE_NUM: f64 = 1.0;
	pub const FALSE_NUM: f64 = 0.0;

	pub fn at_bool(obj: &Object<Boolean>) -> Object<Boolean> {
		obj.duplicate()
	}
	
	pub fn at_num(obj: &Object<Boolean>) -> Object<Number> {
		if obj.is_true() {
			Object::new_number(TRUE_NUM)
		} else {
			Object::new_number(FALSE_NUM)
		}
	}

	pub fn at_text(obj: &Object<Boolean>) -> Object<Text> {
		// this is instead of `is_true().to_string()` in case I want to change something
		if obj.is_true() {
			Object::new_text_str(TRUE_STR)
		} else {
			Object::new_text_str(FALSE_STR)
		}
	}

	pub fn not(obj: &Object<Boolean>) -> Object<Boolean> {
		Object::new_boolean(!obj.is_true())
	}

	pub fn eql(lhs: &Object<Boolean>, rhs: &Object<Boolean>) -> Object<Boolean> {
		Object::new_boolean(lhs.is_true() == rhs.is_true())
	}

	pub fn b_xor(lhs: &Object<Boolean>, rhs: &Object<Boolean>) -> Object<Boolean> {
		Object::new_boolean(lhs.is_true() ^ rhs.is_true())
	}

	pub fn b_and(lhs: &Object<Boolean>, rhs: &Object<Boolean>) -> Object<Boolean> {
		Object::new_boolean(lhs.is_true() & rhs.is_true())
	}

	pub fn b_or(lhs: &Object<Boolean>, rhs: &Object<Boolean>) -> Object<Boolean> {
		Object::new_boolean(lhs.is_true() | rhs.is_true())
	}
}

impl_type! { for Boolean;
	quest_funcs::AT_BOOL => |o, _| Ok(funcs::at_bool(o) as _),
	quest_funcs::AT_NUM => |o, _| Ok(funcs::at_num(o) as _),
	quest_funcs::AT_TEXT => |o, _| Ok(funcs::at_text(o) as _),

	quest_funcs::NOT => |o, _| Ok(funcs::not(o) as _),
	quest_funcs::EQL => |o, a| Ok(funcs::eql(o, &getarg!(a[0] @ to_boolean)?) as _),
	quest_funcs::B_XOR => |o, a| Ok(funcs::b_xor(o, &getarg!(a[0] @ to_boolean)?) as _),
	quest_funcs::B_AND => |o, a| Ok(funcs::b_and(o, &getarg!(a[0] @ to_boolean)?) as _),
	quest_funcs::B_OR => |o, a| Ok(funcs::b_or(o, &getarg!(a[0] @ to_boolean)?) as _),
}

#[cfg(test)]
mod fn_tests {
	use super::funcs;
	use crate::object::Object;

	#[test]
	fn at_bool() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::at_bool(t).is_true(), true);
		assert_eq!(funcs::at_bool(f).is_true(), false);

		assert_obj_duplicated!(t, funcs::at_bool(t));
	}

	#[test]
	fn at_text() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(*funcs::at_text(t).unwrap_data(), funcs::TRUE_STR);
		assert_eq!(*funcs::at_text(f).unwrap_data(), funcs::FALSE_STR);
	}

	#[test]
	fn at_num() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(*funcs::at_num(t).unwrap_data(), funcs::TRUE_NUM);
		assert_eq!(*funcs::at_num(f).unwrap_data(), funcs::FALSE_NUM);
	}

	#[test]
	fn not() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::not(t).is_true(), false);
		assert_eq!(funcs::not(f).is_true(), true);
	}

	#[test]
	fn eql() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::eql(t, t).is_true(), true);
		assert_eq!(funcs::eql(t, f).is_true(), false);
		assert_eq!(funcs::eql(f, t).is_true(), false);
		assert_eq!(funcs::eql(f, f).is_true(), true);
	}

	#[test]
	fn b_xor() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::b_xor(t, t).is_true(), false);
		assert_eq!(funcs::b_xor(t, f).is_true(), true);
		assert_eq!(funcs::b_xor(f, t).is_true(), true);
		assert_eq!(funcs::b_xor(f, f).is_true(), false);
	}

	#[test]
	fn b_and() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::b_and(t, t).is_true(), true);
		assert_eq!(funcs::b_and(t, f).is_true(), false);
		assert_eq!(funcs::b_and(f, t).is_true(), false);
		assert_eq!(funcs::b_and(f, f).is_true(), false);
	}

	#[test]
	fn b_or() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::b_or(t, t).is_true(), true);
		assert_eq!(funcs::b_or(t, f).is_true(), true);
		assert_eq!(funcs::b_or(f, t).is_true(), true);
		assert_eq!(funcs::b_or(f, f).is_true(), false);
	}
}

#[cfg(test)]
mod integration {
	use super::{Boolean, funcs};
	use crate::err::Result;
	use crate::object::Object;
	use crate::object::types::{Text, Number};
	use crate::object::types::quest_funcs::{
		AT_BOOL, AT_TEXT, AT_NUM,
		NOT, EQL, B_XOR, B_AND, B_OR
	};

	#[test]
	fn at_bool() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?, funcs::at_bool(t));
		assert_eq!(f.as_any().call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?, funcs::at_bool(f));

		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?, funcs::at_text(t));
		assert_eq!(f.as_any().call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?, funcs::at_text(f));

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?, funcs::at_num(t));
		assert_eq!(f.as_any().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?, funcs::at_num(f));

		Ok(())
	}

	#[test]
	fn not() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(NOT, &[])?.downcast_or_err::<Boolean>()?, funcs::not(t));
		assert_eq!(f.as_any().call_attr(NOT, &[])?.downcast_or_err::<Boolean>()?, funcs::not(f));

		Ok(())
	}

	#[test]
	fn eql() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(EQL, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::eql(t, t));
		assert_eq!(t.as_any().call_attr(EQL, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::eql(t, f));
		assert_eq!(f.as_any().call_attr(EQL, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::eql(f, t));
		assert_eq!(f.as_any().call_attr(EQL, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::eql(f, f));

		Ok(())
	}

	#[test]
	fn b_xor() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(B_XOR, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_xor(t, t));
		assert_eq!(t.as_any().call_attr(B_XOR, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_xor(t, f));
		assert_eq!(f.as_any().call_attr(B_XOR, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_xor(f, t));
		assert_eq!(f.as_any().call_attr(B_XOR, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_xor(f, f));

		Ok(())
	}

	#[test]
	fn b_and() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(B_AND, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_and(t, t));
		assert_eq!(t.as_any().call_attr(B_AND, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_and(t, f));
		assert_eq!(f.as_any().call_attr(B_AND, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_and(f, t));
		assert_eq!(f.as_any().call_attr(B_AND, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_and(f, f));

		Ok(())
	}

	#[test]
	fn b_or() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(t.as_any().call_attr(B_OR, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_or(t, t));
		assert_eq!(t.as_any().call_attr(B_OR, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_or(t, f));
		assert_eq!(f.as_any().call_attr(B_OR, &[&t.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_or(f, t));
		assert_eq!(f.as_any().call_attr(B_OR, &[&f.as_any()])?.downcast_or_err::<Boolean>()?, funcs::b_or(f, f));

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn new() {
		assert_eq!(Boolean::new(true), Boolean::new(true));
		assert_ne!(Boolean::new(true), Boolean::new(false));
		assert_eq!(Boolean::new(false), Boolean::new(false));
	}

	#[test]
	fn new_boolean() {
		assert_eq!(Object::new(Boolean::new(true)), Object::new_boolean(true));
		assert_eq!(Object::new(Boolean::new(false)), Object::new_boolean(false));
	}

	#[test]
	fn to_boolean() -> Result<()> {
		assert_eq!(*Object::new_boolean(true).as_any().to_boolean()?.unwrap_data(), true);
		assert_eq!(*Object::new_boolean(false).as_any().to_boolean()?.unwrap_data(), false);

		// TODO: make `MyStruct` here so it doesn't rely upon number
		assert_eq!(*Object::new_number(0.0).as_any().to_boolean()?.unwrap_data(), false);
		assert_eq!(*Object::new_number(1.0).as_any().to_boolean()?.unwrap_data(), true);
		
		Ok(())
	}


	#[test]
	fn is_true() {
		assert_eq!(Boolean::new(true).is_true(), true);
		assert_eq!(Boolean::new(false).is_true(), false);
	}

	#[test]
	fn from_and_into() {
		assert_eq!(Boolean::from(true), Boolean::new(true));
		assert_eq!(Boolean::from(false), Boolean::new(false));
		assert_eq!(bool::from(Boolean::new(true)), true);
		assert_eq!(bool::from(Boolean::new(false)), false);
	}

	#[test]
	fn as_ref() {
		assert_eq!(Boolean::new(false).as_ref(), &false);
		assert_eq!(Boolean::new(true).as_ref(), &true);
	}

	#[test]
	fn equality() {
		assert_eq!(Boolean::new(true), Boolean::new(true));
		assert_eq!(Boolean::new(false), Boolean::new(false));
		assert_ne!(Boolean::new(false), Boolean::new(true));
		assert_ne!(Boolean::new(true), Boolean::new(false));
	}
}




