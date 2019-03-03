use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
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
	use crate::object::{Object, AnyObject};
	use crate::err::Result;

	pub fn at_bool(obj: &Object<Boolean>, _: &[&AnyObject]) -> Result<Object<Boolean>> {
		Ok(obj.duplicate())
	}
	
	pub fn at_num(obj: &Object<Boolean>, _: &[&AnyObject]) -> Result<Object<Number>> {
		if obj.is_true() {
			Ok(Object::new_number(1.0))
		} else {
			Ok(Object::new_number(0.0))
		}
	}

	pub fn at_text(obj: &Object<Boolean>, _: &[&AnyObject]) -> Result<Object<Text>> {
		Ok(Object::new_text(obj.is_true().to_string()))
	}

	pub fn not(obj: &Object<Boolean>, _: &[&AnyObject]) -> Result<Object<Boolean>> {
		use crate::object::types::quest_funcs::NOT;
		Ok(Object::new_boolean(!obj.data().read().expect(data_err![read in Boolean, NOT]).is_true()))
	}

	pub fn eql(lhs: &Object<Boolean>, args: &[&AnyObject]) -> Result<Object<Boolean>> {
		let rhs = getarg!(args[0] @ to_boolean)?;
		Ok(Object::new_boolean(lhs.is_true() == rhs.is_true()))
	}

	pub fn b_xor(lhs: &Object<Boolean>, args: &[&AnyObject]) -> Result<Object<Boolean>> {
		let rhs = getarg!(args[0] @ to_boolean)?;
		Ok(Object::new_boolean(lhs.is_true() ^ rhs.is_true()))
	}

	pub fn b_and(lhs: &Object<Boolean>, args: &[&AnyObject]) -> Result<Object<Boolean>> {
		let rhs = getarg!(args[0] @ to_boolean)?;
		Ok(Object::new_boolean(lhs.is_true() & rhs.is_true()))
	}

	pub fn b_or(lhs: &Object<Boolean>, args: &[&AnyObject]) -> Result<Object<Boolean>> {
		let rhs = getarg!(args[0] @ to_boolean)?;
		Ok(Object::new_boolean(lhs.is_true() | rhs.is_true()))
	}
}

impl_type! { for Boolean;
	quest_funcs::AT_BOOL => |o, a| funcs::at_bool(o, a).map(|x| x as _),
	quest_funcs::AT_NUM => |o, a| funcs::at_num(o, a).map(|x| x as _),
	quest_funcs::AT_TEXT => |o, a| funcs::at_text(o, a).map(|x| x as _),

	quest_funcs::NOT => |o, a| funcs::not(o, a).map(|x| x as _),
	quest_funcs::EQL => |o, a| funcs::eql(o, a).map(|x| x as _),
	quest_funcs::B_XOR => |o, a| funcs::b_xor(o, a).map(|x| x as _),
	quest_funcs::B_AND => |o, a| funcs::b_and(o, a).map(|x| x as _),
	quest_funcs::B_OR => |o, a| funcs::b_or(o, a).map(|x| x as _),
}

#[cfg(test)]
mod fn_tests {
	use super::{*, quest_funcs::*};
	use crate::object::types::{Number, Text};
	use crate::err::Error;

	macro_rules! _b_ {
		($bool:expr) => (Object::new_boolean($bool).as_any())
	}
	macro_rules! b {
		($bool:expr) => (Object::new_boolean($bool))
	}

	macro_rules! assert_bool_call_eq {
		($attr:tt $type:ty; $(($obj:expr, $args:tt) => $expected:expr),*) => {
			$(
				assert_eq!(**_b_!($obj).call_attr($attr, &$args)?.downcast_or_err::<$type>()?.data().read().unwrap(), $expected);
			)*
		}
	}

	#[test]
	fn at_bool() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::at_bool(t, &[])?.is_true(), true);
		assert_eq!(funcs::at_bool(f, &[])?.is_true(), false);
		assert_eq!(funcs::at_bool(f, &[&t.as_any()])?.is_true(), false);

		assert_obj_duplicated!(t, funcs::at_bool(t, &[])?);

		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::at_text(t, &[])?.data().read().unwrap().as_ref(), "true");
		assert_eq!(funcs::at_text(f, &[])?.data().read().unwrap().as_ref(), "false");
		assert_eq!(funcs::at_text(f, &[&t.as_any()])?.data().read().unwrap().as_ref(), "false");

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_eq!(funcs::at_num(t, &[])?.data().read().unwrap().as_ref(), &1.0);
		assert_eq!(funcs::at_num(f, &[])?.data().read().unwrap().as_ref(), &0.0);
		assert_eq!(funcs::at_num(f, &[&t.as_any()])?.data().read().unwrap().as_ref(), &0.0);

		Ok(())
	}

	#[test]
	fn equality() -> Result<()> {
		let ref t = Object::new_boolean(true);
		let ref f = Object::new_boolean(false);

		assert_bool_call_eq!(EQL Boolean; 
			(true, [&_b_!(true)]) => true,
			(true, [&_b_!(false)]) => false,
			(false, [&_b_!(true)]) => false,
			(false, [&_b_!(false)]) => true,
			(false, [&_b_!(false), &_b_!(true)]) => true // ensure extra args are ignored
		);

		assert_param_missing!(_b_!(true).call_attr(EQL, &[]));

		Ok(())
	}

	#[test]
	fn not() -> Result<()> {
		assert_bool_call_eq!(NOT Boolean;
			(true, []) => false,
			(false, []) => true,
			(true, [&_b_!(false)]) => false // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn b_xor() -> Result<()> {
		assert_bool_call_eq!(B_XOR Boolean; 
			(true, [&_b_!(true)]) => false,
			(true, [&_b_!(false)]) => true,
			(false, [&_b_!(true)]) => true,
			(false, [&_b_!(false)]) => false,
			(false, [&_b_!(false), &_b_!(true)]) => false // ensure extra args are ignored
		);

		assert_param_missing!(_b_!(true).call_attr(B_XOR, &[]));

		Ok(())
	}

	#[test]
	fn b_and() -> Result<()> {
		assert_bool_call_eq!(B_AND Boolean; 
			(true, [&_b_!(true)]) => true,
			(true, [&_b_!(false)]) => false,
			(false, [&_b_!(true)]) => false,
			(false, [&_b_!(false)]) => false,
			(true, [&_b_!(false), &_b_!(true)]) => false // ensure extra args are ignored
		);

		assert_param_missing!(_b_!(true).call_attr(B_AND, &[]));

		Ok(())
	}

	#[test]
	fn b_or() -> Result<()> {
		assert_bool_call_eq!(B_OR Boolean; 
			(true, [&_b_!(true)]) => true,
			(true, [&_b_!(false)]) => true,
			(false, [&_b_!(true)]) => true,
			(false, [&_b_!(false)]) => false,
			(false, [&_b_!(false), &_b_!(true)]) => false // ensure extra args are ignored
		);

		assert_param_missing!(_b_!(true).call_attr(B_OR, &[]));

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
		assert_eq!(**Object::new_boolean(true).as_any().to_boolean()?.data().read().unwrap(), true);
		assert_eq!(**Object::new_boolean(false).as_any().to_boolean()?.data().read().unwrap(), false);

		// TODO: make `MyStruct` here so it doesn't rely upon number
		assert_eq!(**Object::new_number(0.0).as_any().to_boolean()?.data().read().unwrap(), false);
		assert_eq!(**Object::new_number(1.0).as_any().to_boolean()?.data().read().unwrap(), true);
		
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




