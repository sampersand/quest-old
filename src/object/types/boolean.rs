use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::Result;
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
		self.data().read().expect("read error in Object::is_true").is_true()
	}
}

impl AnyObject {
	pub fn to_boolean(&self) -> Result<Object<Boolean>> {
		self.call_attr("@bool", &[])?
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
	"@bool" => |obj, _| Ok(Object::new_boolean(obj.data().read().expect("read err in Boolean::@bool").is_true())),
	"@num" => |obj, _| Ok(Object::new_number(if obj.is_true() { 1.0 } else { 0.0 })),
	"==" => |obj, args| {
		Ok(Object::new_boolean(obj.is_true() == getarg!(args[0] @ to_boolean)?.is_true()))
	},
	"!" => |obj, _| Ok(Object::new_boolean(!obj.data().read().expect("read err in Boolean::!").is_true())),
	"^" => |obj, args| Ok(Object::new_boolean(obj.is_true() ^ getarg!(args[0] @ to_boolean)?.is_true())),
	"*" => |obj, args| Ok(Object::new_boolean(obj.is_true() & getarg!(args[0] @ to_boolean)?.is_true())),
	"|" => |obj, args| Ok(Object::new_boolean(obj.is_true() | getarg!(args[0] @ to_boolean)?.is_true()))
}

#[cfg(test)]
mod type_tests {
	use super::*;
	use crate::object::types::Number;

	#[test]
	fn at_bool() -> Result<()> {
		let obj = Object::new_boolean(true);
		let dup = obj.call_attr("@bool", &[])?.downcast_or_err::<Boolean>()?;
		assert_eq!(*obj.data().read().unwrap(), *dup.data().read().unwrap());
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));
		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		assert_eq!(**Object::new_boolean(true).call_attr("@num", &[])?.downcast_or_err::<Number>()?.data().read().unwrap(), 1.0);
		assert_eq!(**Object::new_boolean(false).call_attr("@num", &[])?.downcast_or_err::<Number>()?.data().read().unwrap(), 0.0);
		Ok(())
	}

	#[test]
	fn equality() -> Result<()> {
		assert!(!Object::new_boolean(true).call_attr("==", &[&Object::new_boolean(false).as_any()])?.downcast_or_err::<Boolean>()?.is_true());
		assert!(Object::new_boolean(true).call_attr("==", &[&Object::new_boolean(true).as_any()])?.downcast_or_err::<Boolean>()?.is_true());
		Ok(())
	}

	#[test]
	fn negate() -> Result<()> {
		assert!(!Object::new_boolean(true).call_attr("!", &[])?.downcast_or_err::<Boolean>()?.is_true());
		assert!(Object::new_boolean(false).call_attr("!", &[])?.downcast_or_err::<Boolean>()?.is_true());
		Ok(())
	}

	#[test]
	fn xor() -> Result<()> {
		assert!(Object::new_boolean(true).call_attr("^", &[&Object::new_boolean(false).as_any()])?.downcast_or_err::<Boolean>()?.is_true());
		assert!(Object::new_boolean(false).call_attr("^", &[&Object::new_boolean(true).as_any()])?.downcast_or_err::<Boolean>()?.is_true());
		assert!(!Object::new_boolean(true).call_attr("^", &[&Object::new_boolean(true).as_any()])?.downcast_or_err::<Boolean>()?.is_true());
		assert!(!Object::new_boolean(false).call_attr("^", &[&Object::new_boolean(false).as_any()])?.downcast_or_err::<Boolean>()?.is_true());
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








