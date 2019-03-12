use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::Result;
use std::ops::Deref;
use super::quest_funcs;

const NULL_STR: &str = "null";
const NULL_NUM: f64 = 0.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;

impl Object<Null> {
	pub fn new_null() -> Object<Null> {
		Object::new(Null)
	}
}

impl AnyObject {
	pub fn to_null(&self) -> Result<Object<Null>> {
		self.downcast_or_err::<Null>() // we don't have an attr to downcast to
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

mod funcs {
	use super::Null;
	use crate::err::Result;
	use crate::object::{Object, AnyObject};
	use crate::object::types::{Text, Boolean, Number};

	pub fn at_text(_: &Object<Null>) -> Object<Text> {
		Object::new_text_str(super::NULL_STR)
	}

	pub fn at_bool(_: &Object<Null>) -> Object<Boolean> {
		Object::new_boolean(false)
	}

	pub fn at_num(_: &Object<Null>) -> Object<Number> {
		Object::new_number(super::NULL_NUM)
	}

	pub fn eql(_: &Object<Null>, _: &Object<Null>) -> Object<Boolean> {
		Object::new_boolean(true)
	}

	pub fn call(_: &Object<Null>, _: &[&AnyObject]) -> Result<AnyObject> {
		Ok(Object::new_null())
	}
}

impl_type! { for Null;
	quest_funcs::AT_TEXT => |n, _| Ok(funcs::at_text(n)),
	quest_funcs::AT_BOOL => |n, _| Ok(funcs::at_bool(n)),
	quest_funcs::AT_NUM => |n, _| Ok(funcs::at_num(n)),
	quest_funcs::EQL => |n, a| Ok(getarg!(a[0])?.to_null().map(|ref arg| funcs::eql(n, arg)).unwrap_or_else(|_| Object::new_boolean(false))),
	quest_funcs::CALL => funcs::call
}

#[cfg(test)]
mod fn_tests {
	use super::funcs;
	use crate::object::Object;

	#[test]
	fn at_bool() {
		assert_eq!(funcs::at_bool(&Object::new_null()), false);
	}

	#[test]
	fn at_text() {
		assert_eq!(funcs::at_text(&Object::new_null()), super::NULL_STR);
	}

	#[test]
	fn at_num() {
		assert_eq!(funcs::at_num(&Object::new_null()), 0.0);
	}

	#[test]
	fn eql() {
		let ref n = Object::new_null();
		assert_eq!(funcs::eql(n, n), true);
		assert_eq!(funcs::eql(n, &Object::new_null()), true);
	}

	#[test]
	fn call() {
		let ref n = Object::new_null();
		assert!(funcs::call(n, &[]).unwrap().is_null());
		assert!(funcs::call(n, &[&Object::new_null().as_any()]).unwrap().is_null());
	}
}

#[cfg(test)]
mod integration {
	use super::*;
	use crate::err::Result;
	use crate::object::Object;
	use crate::object::types::{Text, Boolean, Number};
	use quest_funcs::*;

	define_blank!(struct Blank;);

	#[test]
	fn at_bool() -> Result<()> {
		let ref n = Object::new_null();

		assert_eq!(n.as_any().call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?, false);
		assert_eq!(n.as_any().call_attr(AT_BOOL, &[&Blank::new_any()])?.downcast_or_err::<Boolean>()?, false);

		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		let ref n = Object::new_null();
		
		assert_eq!(n.as_any().call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?, "null");
		assert_eq!(n.as_any().call_attr(AT_TEXT, &[&Blank::new_any()])?.downcast_or_err::<Text>()?, "null");

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		let ref n = Object::new_null().as_any();
		
		assert_eq!(n.call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?, 0.0);
		assert_eq!(n.call_attr(AT_NUM, &[&Blank::new_any()])?.downcast_or_err::<Number>()?, 0.0);

		Ok(())
	}

	#[test]
	fn eql() -> Result<()> {
		let ref n = Object::new_null().as_any();
		
		assert_eq!(n.call_attr(EQL, &[&n])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(n.call_attr(EQL, &[&Blank::new_any()])?.downcast_or_err::<Boolean>()?, false);
		assert_param_missing!(n.call_attr(EQL, &[]));

		Ok(())
	
	}

	#[test]
	fn call() -> Result<()> {
		let ref n = Object::new_null().as_any();

		assert!(n.call_attr(CALL, &[])?.is_null());
		assert!(n.call_attr(CALL, &[&Blank::new_any()])?.is_null());

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








