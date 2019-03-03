use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::Result;
use std::ops::Deref;
use super::quest_funcs;

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
	use super::{Null, NULL_STR};
	use crate::err::Result;
	use crate::object::{Object, AnyObject};
	use crate::object::types::{Text, Boolean, Number};

	pub fn at_text(_: &Object<Null>) -> Object<Text> {
		Object::new_text_str(NULL_STR)
	}

	pub fn at_bool(_: &Object<Null>) -> Object<Boolean> {
		Object::new_boolean(false)
	}

	pub fn at_num(_: &Object<Null>) -> Object<Number> {
		Object::new_number(std::f64::NAN)
	}

	pub fn eql(_: &Object<Null>, _: &Object<Null>) -> Object<Boolean> {
		Object::new_boolean(true)
	}

	pub fn call(_: &Object<Null>, _: &[&AnyObject]) -> Result<AnyObject> {
		Ok(Object::new_null())
	}
}

impl_type! { for Null;
	quest_funcs::AT_TEXT => |o, _| Ok(funcs::at_text(o)),
	quest_funcs::AT_BOOL => |o, _| Ok(funcs::at_bool(o)),
	quest_funcs::AT_NUM => |o, _| Ok(funcs::at_num(o)),
	quest_funcs::EQL => |o, a| Ok(getarg!(a[0])?.to_null().map(|ref arg| funcs::eql(o, arg)).unwrap_or_else(|_| Object::new_boolean(false))),
	quest_funcs::CALL => |o, a| funcs::call(o, a)
}

#[cfg(test)]
mod fn_tests {
	use super::funcs;
	use crate::object::Object;

	#[test]
	fn at_bool() {
		let ref n = Object::new_null();
		assert_eq!(funcs::at_bool(n), false);
	}

	#[test]
	fn at_text() {
		let ref n = Object::new_null();
		assert_eq!(funcs::at_text(n), super::NULL_STR);
	}

	#[test]
	fn at_num() {
		let ref n = Object::new_null();
		assert!(funcs::at_num(n).is_nan());
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
		match funcs::call(n, &[]) {
			Ok(ref obj) if obj.is_null() => {},
			other => panic!("bad result: {:?}", other)
		}

		match funcs::call(n, &[&Object::new_null().as_any()]) {
			Ok(ref obj) if obj.is_null() => {},
			other => panic!("bad result: {:?}", other)
		}
	}
}

#[cfg(test)]
mod integration {
	use super::funcs;
	use crate::err::Result;
	use crate::object::Object;
	use crate::object::types::{Text, Boolean, Number};
	use crate::object::types::quest_funcs::{
		AT_BOOL, AT_TEXT, AT_NUM,
		EQL, CALL
	};

	#[test]
	fn at_bool() -> Result<()> {
		let ref n = Object::new_null();
		
		assert_eq!(n.as_any().call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?, funcs::at_bool(n));

		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		let ref n = Object::new_null();
		
		assert_eq!(n.as_any().call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?, funcs::at_text(n));

		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		let ref n = Object::new_null();
		
		assert!(n.as_any().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?.is_nan());

		Ok(())
	}

	#[test]
	fn eql() -> Result<()> {
		let ref n = Object::new_null();
		let ref n2 = Object::new_null();
		
		assert_eq!(n.as_any().call_attr(EQL, &[&n.as_any()])?.downcast_or_err::<Boolean>()?, funcs::eql(n, n));
		assert_eq!(n.as_any().call_attr(EQL, &[&n2.as_any()])?.downcast_or_err::<Boolean>()?, funcs::eql(n, n2));

		define_blank!(struct Blank;);
		assert_eq!(n.as_any().call_attr(EQL, &[&Blank::new_any()])?.downcast_or_err::<Boolean>()?, false);

		assert_param_missing!(n.as_any().call_attr(EQL, &[]));

		Ok(())
	
	}

	#[test]
	fn call() -> Result<()> {
		let ref n = Object::new_null();
		let ref n2 = Object::new_null().as_any();

		assert_eq!(n.as_any().call_attr(CALL, &[])?, funcs::call(n, &[])?);
		assert_eq!(n.as_any().call_attr(CALL, &[n2])?, funcs::call(n, &[n2])?);

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








