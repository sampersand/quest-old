use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::Result;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;

impl Object<Null> {
	pub fn new_null() -> Object<Null> {
		Object::new(Null)
	}
}

impl AnyObject {
	pub fn to_null(&self) -> Result<Object<Null>> {
		self//.call_attr("@bool", &[])?
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
		write!(f, "null")
	}
}

// fn "@text" (_) {
// 		"null".to_string().into_object()
// 	}

// 	fn "@bool" (_) {
// 		false.into_object()
// 	}

// 	fn "==" (_this, rhs) {
// 		rhs.is_null().into_object()
// 	}

// 	fn "()" (@_this) { Object::new_null() } // for stuff like if(foo {12})!
// }
impl_type! { for Null;
	"@text" => |_, _| Ok(Object::new_text_str("null")),
	"@bool" => |_, _| Ok(Object::new_boolean(false)),
	"@num" => |_, _| Ok(Object::new_number(std::f64::NAN)),
	"!" => |_, _| Ok(Object::new_boolean(true)),
	"==" => |_, args| Ok(Object::new_boolean(getarg!(args[0])?.is_null())),
	"()" => |_, _| Ok(Object::new_null()) // executing null gives you null
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
				assert_eq!(**n!().call_attr($attr, &$args)?.downcast_or_err::<$type>()?.data().read().unwrap(), $expected);
			)*
		}
	}


	#[test]
	fn at_bool() -> Result<()> {
		assert_null_call_eq!("@bool" Boolean;
			(_, []) => false,
			(_, [&Object::new_number(12.3).as_any()]) => false // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_null_call_eq!("@text" Text;
			(_, []) => *"null",
			(_, [&Object::new_number(12.3).as_any()]) => *"null" // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		assert!(n!().call_attr("@num", &[])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		// ensure extra args are ignored
		assert!(n!().call_attr("@num", &[&Object::new_number(12.3).as_any()])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		Ok(())
	}


	#[test]
	fn equality() -> Result<()> {
		assert_null_call_eq!("==" Boolean;
			(_, [&n!()]) => true,
			(_, [&Object::new_boolean(false).as_any()]) => false, 
			(_, [&n!(), &Object::new_number(12.3).as_any()]) => true // ensure extra args are ignored
		);

		// check to see if too few args are passed it handles it right
		match n!().call_attr("==", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!("got bad err")
		};

		Ok(())
	}

	#[test]
	fn negate() -> Result<()> {
		assert_null_call_eq!("!" Boolean;
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








