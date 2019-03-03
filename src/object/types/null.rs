use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::Result;
use std::ops::Deref;
use super::quest_funcs::{
	AT_TEXT, AT_BOOL, AT_NUM,
	NOT, EQL, CALL
};

const NULL_STR: &str = "null";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;

impl Object<Null> {
	pub fn new_null() -> Object<Null> {
		Object::new(Null)
	}
}

impl AnyObject {
	pub fn to_null(&self) -> Result<Object<Null>> {
		self//.call_attr(AT_BOOL, &[])?
			.downcast_or_err::<Null>() // we don't have an attr to downcast to
	}
	pub fn is_null(&self) -> bool {
		self.downcast::<Null>().is_some()
	}

}

impl From<()> for Null {
	fn from(_: ()) -> Null {
		Null
	}
}

impl From<Null> for () {
	fn from(_: Null) -> () {
		()
	}
}

impl Display for Null {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", NULL_STR)
	}
}

impl_type! { for Null;
	AT_TEXT => |_, _| Ok(Object::new_text_str(NULL_STR)),
	AT_BOOL => |_, _| Ok(Object::new_boolean(false)),
	AT_NUM => |_, _| Ok(Object::new_number(std::f64::NAN)),
	NOT => |_, _| Ok(Object::new_boolean(true)),
	EQL => |_, args| Ok(Object::new_boolean(getarg!(args[0])?.is_null())),
	CALL => |_, _| Ok(Object::new_null()) // executing null gives you null
}

#[cfg(test)]
mod fn_tests {
	use super::*;
	use crate::object::types::{Number, Text, Boolean};
	use crate::err::Error;

	macro_rules! n {
		() => (Object::new_null().as_any())
	}

	macro_rules! assert_null_call_eq {
		($attr:tt $type:ty; $((_, $args:tt) => $expected:expr),*) => {
			$(
				assert_eq!(*n!().call_attr($attr, &$args)?.downcast_or_err::<$type>()?.unwrap_data(), $expected);
			)*
		}
	}


	#[test]
	fn at_bool() -> Result<()> {
		assert_null_call_eq!(AT_BOOL Boolean;
			(_, []) => false,
			(_, [&Object::new_number(12.3).as_any()]) => false // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_null_call_eq!(AT_TEXT Text;
			(_, []) => *NULL_STR,
			(_, [&Object::new_number(12.3).as_any()]) => *NULL_STR // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		assert!(n!().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		// ensure extra args are ignored
		assert!(n!().call_attr(AT_NUM, &[&Object::new_number(12.3).as_any()])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());

		Ok(())
	}


	#[test]
	fn equality() -> Result<()> {
		assert_null_call_eq!(EQL Boolean;
			(_, [&n!()]) => true,
			(_, [&Object::new_boolean(false).as_any()]) => false, 
			(_, [&n!(), &Object::new_number(12.3).as_any()]) => true // ensure extra args are ignored
		);

		assert_param_missing!(n!().call_attr(EQL, &[]));

		Ok(())
	}

	#[test]
	fn negate() -> Result<()> {
		assert_null_call_eq!(NOT Boolean;
			(_, []) => true,
			(_, [&Object::new_number(12.3).as_any()]) => true // ensure extra args are ignored
		);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		assert_eq!(Null, Null);
	}

	#[test]
	fn new_null() {
		assert_eq!(Object::new(Null), Object::new_null());
	}

	#[test]
	fn to_null() -> Result<()> {
		Object::new_null().as_any().to_null()?; // ignore the result
		
		Ok(())
	}


	#[test]
	fn is_null()  {
		assert!(Object::new_null().as_any().is_null());
		assert!(!Object::new_boolean(false).as_any().is_null());
	}

	#[test]
	fn from_and_into() {
		assert_eq!(Null::from(()), Null);
		assert_eq!(<()>::from(Null), ());
	}
}








