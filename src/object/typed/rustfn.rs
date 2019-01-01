use super::TypedObject;
use crate::{Object, Result};
use lazy_static::lazy_static;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Display, Formatter};

type Inner = fn(&[&Object]) -> Result;

#[derive(Clone, Copy)]
pub struct RustFn {
	name: &'static str,
	func: Inner
}

impl RustFn {
	#[inline]
	pub fn new(name: &'static str, func: Inner) -> Self {
		RustFn { name, func }
	}

	pub fn call(&self, args: &[&Object]) -> Result {
		(self.func)(args)
	}
}

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "RustFn({:?}, {:p})", self.name, self.func as *const ())
		} else {
			write!(f, "RustFn({:?})", self.name)
		}
	}
}

impl Display for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<rustfn {}>", self.name)
	}
}


impl Eq for RustFn {}
impl PartialEq for RustFn {
	fn eq(&self, other: &RustFn) -> bool {
		let func_eq = self.func as usize == other.func as usize;
		debug_assert_eq!(self.name == other.name, func_eq, "function {:?} is incompatible with {:?}", self, other);
		func_eq
	}
}

impl Hash for RustFn {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.func as usize).hash(h)
	}
}

impl TypedObject {
	pub fn new_rustfn(name: &'static str, func: Inner) -> Self {
		TypedObject::new(RustFn::new(name, func))
	}
}

impl Object {
	pub fn new_rustfn(name: &'static str, func: Inner) -> Self {
		Object::new(TypedObject::new_rustfn(name, func))
	}
}

impl_typed_object!(RustFn, _ , downcast_rustfn, is_rustfn);


impl_type! { for RustFn, downcast_fn=downcast_rustfn;
	fn "name" (this) {
		this.name.to_string().into_object()
	}

	fn "@text" (this) {
		format!("{}", this).into_object()
	}

	fn "()" (this) args {
		this.call(args)?
	}
}