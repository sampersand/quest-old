use super::{TypedObject, Type, Types};
use crate::{Shared, Object, Result};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};

type Inner = fn(&[&Shared<Object>]) -> Result;

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

	pub fn call(&self, args: &[&Shared<Object>]) -> Result {
		(self.func)(args)
	}
}

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "RustFn {{ name: {}, func: {:p} }}", self.name, self.func as *const ())
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

impl Type for RustFn {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new({
				Object::new(crate::collections::Map::default())
			});
		}
		Shared::new(ParentalMap::new_default(PARENT.clone()))
	}
}


impl TypedObject {
	pub fn new_rustfn(name: &'static str, func: Inner) -> Self {
		TypedObject::new(RustFn::new(name, func))
	}
}

impl_typed_object!(RustFn, _ , downcast_rustfn);