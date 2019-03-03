use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::Result;
use std::ops::Deref;
use super::quest_funcs::{
	AT_BOOL, AT_NUM, AT_TEXT,
	NOT, EQL,
	B_XOR, B_AND, B_OR
};

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
		self.call_attr(AT_BOOL, &[])?
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

impl_type! { for Boolean;
	AT_BOOL => |obj, _| Ok(Object::new_boolean(obj.data().read().expect("read err in Boolean::@bool").is_true())),
	AT_NUM => |obj, _| Ok(Object::new_number(if obj.is_true() { 1.0 } else { 0.0 })),
	AT_TEXT => |obj, _| Ok(Object::new_text(obj.is_true().to_string())),

	NOT => |obj, _| Ok(Object::new_boolean(!obj.data().read().expect("read err in Boolean::!").is_true())),
	EQL => |obj, args| Ok(Object::new_boolean(obj.is_true() == getarg!(args[0] @ to_boolean)?.is_true())),
	B_XOR => |obj, args| Ok(Object::new_boolean(obj.is_true() ^ getarg!(args[0] @ to_boolean)?.is_true())),
	B_AND => |obj, args| Ok(Object::new_boolean(obj.is_true() & getarg!(args[0] @ to_boolean)?.is_true())),
	B_OR => |obj, args| Ok(Object::new_boolean(obj.is_true() | getarg!(args[0] @ to_boolean)?.is_true()))
}

#[cfg(test)]
mod fn_tests {
	use super::*;
	use crate::object::types::{Number, Text};
	use crate::err::Error;

	macro_rules! b {
		($bool:expr) => (Object::new_boolean($bool).as_any())
	}

	macro_rules! assert_bool_call_eq {
		($attr:tt $type:ty; $(($obj:expr, $args:tt) => $expected:expr),*) => {
			$(
				assert_eq!(**b!($obj).call_attr($attr, &$args)?.downcast_or_err::<$type>()?.data().read().unwrap(), $expected);
			)*
		}
	}


	#[test]
	fn at_bool() -> Result<()> {
		assert_bool_call_eq!(AT_BOOL Boolean;
			(true, []) => true,
			(false, []) => false,
			(true, [&b!(false)]) => true // ensure extra args are ignored
		);

		// ensnure that the map isn't the same object
		let obj = Object::new_boolean(true);
		let dup = obj.as_any().call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?;
		assert_eq!(*obj.data().read().unwrap(), *dup.data().read().unwrap());
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));
		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_bool_call_eq!(AT_TEXT Text; 
			(true, []) => *"true",
			(false, []) => *"false",
			(true, [&b!(false)]) => *"true" // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		assert_bool_call_eq!(AT_NUM Number; 
			(true, []) => 1.0,
			(false, []) => 0.0,
			(true, [&b!(false)]) => 1.0 // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn equality() -> Result<()> {
		assert_bool_call_eq!(EQL Boolean; 
			(true, [&b!(true)]) => true,
			(true, [&b!(false)]) => false,
			(false, [&b!(true)]) => false,
			(false, [&b!(false)]) => true,
			(false, [&b!(false), &b!(true)]) => true // ensure extra args are ignored
		);

		assert_param_missing!(b!(true).call_attr(EQL, &[]));

		Ok(())
	}

	#[test]
	fn not() -> Result<()> {
		assert_bool_call_eq!(NOT Boolean;
			(true, []) => false,
			(false, []) => true,
			(true, [&b!(false)]) => false // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn b_xor() -> Result<()> {
		assert_bool_call_eq!(B_XOR Boolean; 
			(true, [&b!(true)]) => false,
			(true, [&b!(false)]) => true,
			(false, [&b!(true)]) => true,
			(false, [&b!(false)]) => false,
			(false, [&b!(false), &b!(true)]) => false // ensure extra args are ignored
		);

		assert_param_missing!(b!(true).call_attr(B_XOR, &[]));

		Ok(())
	}

	#[test]
	fn b_and() -> Result<()> {
		assert_bool_call_eq!(B_AND Boolean; 
			(true, [&b!(true)]) => true,
			(true, [&b!(false)]) => false,
			(false, [&b!(true)]) => false,
			(false, [&b!(false)]) => false,
			(true, [&b!(false), &b!(true)]) => false // ensure extra args are ignored
		);

		assert_param_missing!(b!(true).call_attr(B_AND, &[]));

		Ok(())
	}

	#[test]
	fn b_or() -> Result<()> {
		assert_bool_call_eq!(B_OR Boolean; 
			(true, [&b!(true)]) => true,
			(true, [&b!(false)]) => true,
			(false, [&b!(true)]) => true,
			(false, [&b!(false)]) => false,
			(false, [&b!(false), &b!(true)]) => false // ensure extra args are ignored
		);

		assert_param_missing!(b!(true).call_attr(B_OR, &[]));

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




