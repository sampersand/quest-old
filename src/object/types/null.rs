use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::convert::TryFrom;
use crate::object::{literals, Object, AnyObject};
use crate::error::{Result, Error};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;

impl Object<Null> {
	pub fn new_null() -> Object<Null> {
		Object::new(Null)
	}
}

impl TryFrom<AnyObject> for Object<Null> {
	type Error = Error;
	fn try_from(obj: AnyObject) -> Result<Object<Null>> {
		obj.to_null()
	}
}

impl AnyObject {
	#[deprecated]
	pub fn to_null(&self) -> Result<Object<Null>> {
		self.downcast_or_err::<Null>() // there's no `at_null` so just downcasting will do
	}

	pub fn is_null(&self) -> bool {
		self.is::<Null>()
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

mod funcs {
	use super::Null;
	use crate::error::Result;
	use crate::object::{Object, AnyObject};
	use crate::object::types::{Text, List, Map, Boolean, Number};

	pub fn at_text(_: &Object<Null>) -> Object<Text> {
		Object::new_text_str("null")
	}

	pub fn at_bool(_: &Object<Null>) -> Object<Boolean> {
		Object::new_boolean(false)
	}

	pub fn at_num(_: &Object<Null>) -> Object<Number> {
		Object::new_number(0.0)
	}

	pub fn at_list(_: &Object<Null>) -> Object<List> {
		Object::new_list(List::empty())
	}

	pub fn at_map(_: &Object<Null>) -> Object<Map> {
		Object::new_map(Map::empty())
	}

	pub fn eql(_: &Object<Null>, _: &Object<Null>) -> Object<Boolean> {
		Object::new_boolean(true)
	}

	pub fn call(_: &Object<Null>, _: &[&AnyObject]) -> Result<AnyObject> {
		Ok(Object::new_null())
	}
}

impl_type! { for Null;
	literals::AT_TEXT => |n, _| Ok(funcs::at_text(n)),
	literals::AT_BOOL => |n, _| Ok(funcs::at_bool(n)),
	literals::AT_NUM => |n, _| Ok(funcs::at_num(n)),
	literals::AT_LIST => |n, _| Ok(funcs::at_list(n)),
	literals::AT_MAP => |n, _| Ok(funcs::at_map(n)),

	literals::EQL => |n, a| Ok(getarg!(a[0] required: Null)?.map(|n2| funcs::eql(n, n2)).unwrap_or_else(|| Object::new_boolean(false))),
	literals::CALL => funcs::call
}

#[cfg(test)]
mod fn_tests {
	use super::funcs;
	use crate::object::{Object, types};

	#[test]
	fn at_bool() {
		assert_eq!(funcs::at_bool(&Object::new_null()), false);
	}

	#[test]
	fn at_text() {
		assert_eq!(funcs::at_text(&Object::new_null()), "null");
	}

	#[test]
	fn at_num() {
		assert_eq!(funcs::at_num(&Object::new_null()), 0.0);
	}

	#[test]
	fn at_map() {
		assert_eq!(funcs::at_map(&Object::new_null()), types::Map::empty());
	}

	#[test]
	fn at_list() {
		assert_eq!(funcs::at_list(&Object::new_null()), types::List::empty());
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
	use crate::error::Result;
	use crate::object::Object;
	use crate::object::types::{Text, Boolean, Number, Map, List};
	use literals::*;

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
	fn at_map() -> Result<()> {
		let ref n = Object::new_null().as_any();

		assert_eq!(n.call_attr(AT_MAP, &[])?.downcast_or_err::<Map>()?, Map::empty());
		assert_eq!(n.call_attr(AT_MAP, &[&Blank::new_any()])?.downcast_or_err::<Map>()?, Map::empty());

		Ok(())
	}

	#[test]
	fn at_list() -> Result<()> {
		let ref n = Object::new_null().as_any();

		assert_eq!(n.call_attr(AT_LIST, &[])?.downcast_or_err::<List>()?, List::empty());
		assert_eq!(n.call_attr(AT_LIST, &[&Blank::new_any()])?.downcast_or_err::<List>()?, List::empty());

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








