use crate::object::{Object, AnyObject};
use crate::err::{Error, Result};
use std::any::Any;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};

use super::quest_funcs::{
	AT_TEXT, CALL
};

type Inner = dyn Fn(&AnyObject, &[&AnyObject]) -> Result<AnyObject> + Send + Sync;

pub struct RustFn {
	name: Option<&'static str>,
	func: Box<Inner>
}

impl RustFn {
	pub fn new<F, T>(func: F) -> RustFn
			where F: Fn(&Object<T>, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static,
			      T: Send + Sync + 'static {
		RustFn::_new(None, func)
	}

	pub fn new_named<F, T>(name: &'static str, func: F) -> RustFn
			where F: Fn(&Object<T>, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static,
			      T: Send + Sync + 'static {
		RustFn::_new(Some(name), func)
	}

	fn _new<F, T>(name: Option<&'static str>, func: F) -> RustFn
			where F: Fn(&Object<T>, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static,
			      T: Send + Sync + 'static {
		RustFn {
			name,
			func: Box::new(move |obj, args| (func)(&obj.downcast_or_err::<T>()?, args))
		}
	}

	pub fn new_untyped<F>(func: F) -> RustFn
			where F: Fn(&AnyObject, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static {
		RustFn::_new_untyped(None, func)
	}

	pub fn new_named_untyped<F>(name: &'static str, func: F) -> RustFn
			where F: Fn(&AnyObject, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static {
		RustFn::_new_untyped(Some(name), func)
	}

	fn _new_untyped<F>(name: Option<&'static str>, func: F) -> RustFn
			where F: Fn(&AnyObject, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static {
		RustFn { name, func: Box::new(func) }
	}


	pub fn name(&self) -> Option<&'static str> {
		self.name
	}

	pub fn call(&self, obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
		(self.func)(obj, args)
	}
}

impl Object<RustFn> {
	pub fn new_rustfn<F, T>(func: F) -> Object<RustFn>
			where F: Fn(&Object<T>, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static,
			      T: Send + Sync + 'static {
		Object::new(RustFn::new(func))
	}

	pub fn new_named_rustfn<F, T>(name: &'static str, func: F) -> Object<RustFn>
			where F: Fn(&Object<T>, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static,
			      T: Send + Sync + 'static {
		Object::new(RustFn::new_named(name, func))
	}

	pub fn new_untyped_rustfn<F>(func: F) -> Object<RustFn>
			where F: Fn(&AnyObject, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static {
		Object::new(RustFn::new_untyped(func))
	}

	pub fn new_named_untyped_rustfn<F>(name: &'static str, func: F) -> Object<RustFn>
			where F: Fn(&AnyObject, &[&AnyObject]) -> Result<AnyObject>,
			      F: Send + Sync + 'static {
		Object::new(RustFn::new_named_untyped(name, func))
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	fn eq(&self, other: &RustFn) -> bool {
		self.name == other.name && std::ptr::eq(&*self.func, &*other.func)
	}
}

impl Hash for RustFn {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.name.hash(h);
		(self.func.as_ref() as *const _ as *const ()).hash(h);
	}
}

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("RustFn")
		 .field("name", &self.name)
		 .field("func", &crate::util::PtrFormatter((self.func.as_ref() as *const _) as *const () as usize))
		 .finish()
	}
}

impl_type! { for RustFn;
	// AT_TEXT => |obj, _| Ok(Object::new_text(format!("{:?}", *obj.data().read().expect("read error in RustFn::@text")))),
	// CALL => |obj, args| obj.as_any().call_attr(CALL, args)
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::object::types::Number;

	#[test]
	fn new() {
		assert_eq!(RustFn::new::<_, !>(|_, _| unreachable!()).name(), None);
		assert_eq!(RustFn::new_named::<_, !>("hi there", |_, _| unreachable!()).name(), Some("hi there"));

		// let f: fn(&Object<!>, &[&AnyObject]) -> Result<AnyObject> = |_, _| unreachable!();
		// assert_eq!(RustFn::new::<_, !>(f), RustFn::new::<_,!>(f));
	}

	#[test]
	fn untyped() {
		assert_eq!(RustFn::new_untyped(|_, _| unreachable!()).name(), None);
		assert_eq!(RustFn::new_named_untyped("hi there", |_, _| unreachable!()).name(), Some("hi there"));

		// let f: fn(&AnyObject, &[&AnyObject]) -> Result<AnyObject> = |_, _| unreachable!();
		// assert_eq!(RustFn::new_untyped(f), RustFn::new_untyped(f));
	}

	#[test]
	fn call_valid() -> Result<()> {
		let func = RustFn::new::<_, Number>(|num, _| Ok(Object::new_number(*num.unwrap_data() + 1.0)));

		assert_eq!(&func.call(&Object::new_number(123.0).as_any(), &[])?, &Object::new_number(124.0).as_any());
		Ok(())
	}

	#[test]
	fn call_wrong_self() {
		let func = RustFn::new::<_, !>(|_, _| unreachable!());
		match func.call(&Object::new_variable("lol error").as_any(), &[]).unwrap_err() {
			Error::CastError { .. } => {},
			other => panic!("Unexpected error returned: {:?}", other)
		}
	}

	#[test]
	fn call_function_err() {
		let func = RustFn::new::<_, Number>(|_, _| Err(Error::__Testing));

		match func.call(&Object::new_number(1.0).as_any(), &[]).unwrap_err() {
			Error::__Testing => {},
			other => panic!("Unexpected error returned: {:?}", other)
		}
	}

	#[test]
	fn call_untyped() -> Result<()> {
		let func = RustFn::new_untyped(|val, _| {
			Ok(Object::new_boolean(val.data().read().unwrap().is::<Number>()))
		});

		assert_eq!(&func.call(&Object::new_number(123.0).as_any(), &[])?, &Object::new_boolean(true).as_any());
		assert_eq!(&func.call(&Object::new_variable("A").as_any(), &[])?, &Object::new_boolean(false).as_any());

		Ok(())
	}
}






