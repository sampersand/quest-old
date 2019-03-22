use std::fmt::{self, Display, Formatter};
use crate::object::{literals, Object, AnyObject};
use crate::err::{Result, Error};
use std::convert::TryFrom;
use std::ops::Deref;

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
		*self == true
	}
}

impl TryFrom<&'_ AnyObject> for Object<Boolean> {
	type Error = Error;
	fn try_from(obj: &AnyObject) -> Result<Object<Boolean>> {
		obj.call_attr(literals::AT_BOOL, &[])?.downcast_or_err::<Boolean>()
	}
}

impl AnyObject {
	pub fn to_boolean(&self) -> Result<Object<Boolean>> {
		self.call_attr(literals::AT_BOOL, &[])?.downcast_or_err::<Boolean>()
	}
}

impl PartialEq<bool> for Object<Boolean> {
	fn eq(&self, rhs: &bool) -> bool {
		self.data().read().expect("read error in Object<Boolean>::eq").as_ref() == rhs
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
		Display::fmt(if self.0 { "true" } else { "false" }, f)
	}
}

mod funcs {
	use super::Boolean;
	use crate::object::types::{Number, Text};
	use crate::object::Object;

	pub fn at_bool(obj: &Object<Boolean>) -> Object<Boolean> {
		obj.duplicate()
	}
	
	pub fn at_num(obj: &Object<Boolean>) -> Object<Number> {
		if obj.is_true() {
			Object::new_number(1.0)
		} else {
			Object::new_number(0.0)
		}
	}

	pub fn at_text(obj: &Object<Boolean>) -> Object<Text> {
		Object::new_text_str(if obj.is_true() { "true" } else { "false" })
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
	literals::AT_BOOL => |b, _| Ok(funcs::at_bool(b)),
	literals::AT_NUM => |b, _| Ok(funcs::at_num(b)),
	literals::AT_TEXT => |b, _| Ok(funcs::at_text(b)),

	literals::NOT => |b, _| Ok(funcs::not(b)),
	literals::EQL => |b, a| Ok(funcs::eql(b, &getarg!(a[0]: Boolean)?)),
	literals::B_XOR => |b, a| Ok(funcs::b_xor(b, &getarg!(a[0] @ to_boolean)?)),
	literals::B_AND => |b, a| Ok(funcs::b_and(b, &getarg!(a[0] @ to_boolean)?)),
	literals::B_OR => |b, a| Ok(funcs::b_or(b, &getarg!(a[0] @ to_boolean)?)),
}

#[cfg(test)]
mod fn_tests {
	use super::funcs;
	use crate::object::Object;

	#[test]
	fn at_bool() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::at_bool(t), true);
		assert_eq!(funcs::at_bool(f), false);

		assert_obj_duplicated!(t, funcs::at_bool(t));
	}

	#[test]
	fn at_text() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::at_text(t), "true");
		assert_eq!(funcs::at_text(f), "false");
	}

	#[test]
	fn at_num() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::at_num(t), 1.0);
		assert_eq!(funcs::at_num(f), 0.0);

	}

	#[test]
	fn not() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::not(t), false);
		assert_eq!(funcs::not(f), true);
	}

	#[test]
	fn eql() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::eql(t, t), true);
		assert_eq!(funcs::eql(t, f), false);
		assert_eq!(funcs::eql(f, t), false);
		assert_eq!(funcs::eql(f, f), true);
	}

	#[test]
	fn b_xor() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::b_xor(t, t), false);
		assert_eq!(funcs::b_xor(t, f), true);
		assert_eq!(funcs::b_xor(f, t), true);
		assert_eq!(funcs::b_xor(f, f), false);
	}

	#[test]
	fn b_and() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::b_and(t, t), true);
		assert_eq!(funcs::b_and(t, f), false);
		assert_eq!(funcs::b_and(f, t), false);
		assert_eq!(funcs::b_and(f, f), false);
	}

	#[test]
	fn b_or() {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::b_or(t, t), true);
		assert_eq!(funcs::b_or(t, f), true);
		assert_eq!(funcs::b_or(f, t), true);
		assert_eq!(funcs::b_or(f, f), false);
	}
}

#[cfg(test)]
mod integration {
	use super::{Boolean, funcs, literals};
	use crate::err::Result;
	use crate::object::Object;
	use crate::object::types::{Text, Number};
	use literals::*;

	#[test]
	fn at_bool() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(f.call_attr(AT_BOOL, &[t])?.downcast_or_err::<Boolean>()?, false);

		assert_obj_duplicated!(t.downcast_or_err::<Boolean>()?, t.call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?);
		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?, "true");
		assert_eq!(f.call_attr(AT_TEXT, &[t])?.downcast_or_err::<Text>()?, "false");

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?, 1.0);
		assert_eq!(f.call_attr(AT_NUM, &[t])?.downcast_or_err::<Number>()?, 0.0);

		Ok(())
	}

	#[test]
	fn not() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(NOT, &[])?.downcast_or_err::<Boolean>()?, false);
		assert_eq!(f.call_attr(NOT, &[t])?.downcast_or_err::<Boolean>()?, true);

		Ok(())
	}

	#[test]
	fn eql() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(EQL, &[t])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(t.call_attr(EQL, &[f, t])?.downcast_or_err::<Boolean>()?, false);
		assert_param_missing!(t.call_attr(EQL, &[]));

		Ok(())
	}

	#[test]
	fn b_xor() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(B_XOR, &[t])?.downcast_or_err::<Boolean>()?, false);
		assert_eq!(t.call_attr(B_XOR, &[f, t])?.downcast_or_err::<Boolean>()?, true);
		assert_param_missing!(t.call_attr(B_XOR, &[]));

		Ok(())
	}

	#[test]
	fn b_and() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(B_AND, &[t])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(t.call_attr(B_AND, &[f, t])?.downcast_or_err::<Boolean>()?, false);
		assert_param_missing!(t.call_attr(B_AND, &[]));

		Ok(())
	}

	#[test]
	fn b_or() -> Result<()> {
		let ref t = Object::new_boolean(true).as_any();
		let ref f = Object::new_boolean(false).as_any();

		assert_eq!(t.call_attr(B_OR, &[t])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(t.call_attr(B_OR, &[f, t])?.downcast_or_err::<Boolean>()?, true);
		assert_param_missing!(t.call_attr(B_OR, &[]));

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn make_sure_not_nan() {
		assert!(!1.0.is_nan());
		assert!(!0.0.is_nan());
	}

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
		assert_eq!(Object::new_boolean(true).as_any().to_boolean()?, true);
		assert_eq!(Object::new_boolean(false).as_any().to_boolean()?, false);

		// TODO: make `MyStruct` here so it doesn't rely upon number
		assert_eq!(Object::new_number(0.0).as_any().to_boolean()?, false);
		assert_eq!(Object::new_number(1.0).as_any().to_boolean()?, true);
		
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




