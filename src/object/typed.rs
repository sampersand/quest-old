#[macro_use]
mod macros;

mod basic;
mod pristine;

mod bool;
mod null;
mod num;
mod text;
mod var;
mod rustfn;

pub(super) use self::{rustfn::RustFn, var::Var};

use crate::shared::Shared;
use crate::object::Object;
use crate::collections::{Collection, Mapping};
use std::fmt::{self, Display, Formatter};

#[cfg(not(feature = "fine-debug"))]
use std::fmt::Debug;


#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fine-debug", derive(Debug))]
enum Types {
	Null,
	Bool(bool::Bool),
	Num(num::Num),
	Text(text::Text),
	Var(var::Var),
	RustFn(rustfn::RustFn)
}

trait Type : Into<Types> {
	fn create_mapping() -> Shared<dyn Mapping>;
}


#[derive(Clone)]//, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "fine-debug", derive(Debug))]
pub struct TypedObject {
	data: Types,
	map: Shared<dyn Mapping>
}

impl TypedObject {
	fn new<T: Type>(obj: T) -> Self {
		TypedObject {
			data: obj.into(),
			map: T::create_mapping(),
		}
	}

	pub fn objectify(self) -> Object {
		Object::new(self)
	}
}

impl Display for Types {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Types::Null => Display::fmt(&self::null::Null, f),
			Types::Bool(ref bool) => Display::fmt(bool, f),
			Types::Num(ref num) => Display::fmt(num, f),
			Types::Text(ref text) => Display::fmt(text, f),
			Types::Var(ref var) => Display::fmt(var, f),
			Types::RustFn(ref rustfn) => Display::fmt(rustfn, f),
		}
	}
}

#[cfg(not(feature = "fine-debug"))]
impl Debug for Types {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Types::Null => Debug::fmt(&self::null::Null, f),
			Types::Bool(ref bool) => Debug::fmt(bool, f),
			Types::Num(ref num) => Debug::fmt(num, f),
			Types::Text(ref text) => Debug::fmt(text, f),
			Types::Var(ref var) => Debug::fmt(var, f),
			Types::RustFn(ref rustfn) => Debug::fmt(rustfn, f),
		}
	}
}

impl Display for TypedObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.data, f)
	}
}

#[cfg(not(feature = "fine-debug"))]
impl Debug for TypedObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			let mut ds = f.debug_struct("TypedObject");
			ds.field("data", &self.data);
			if !self.map.is_empty() {
				ds.field("map", &self.map);
			}
			return ds.finish();
		}

		if self.map.is_empty() {
			Debug::fmt(&self.data, f)
		} else {
			write!(f, "T{{ data: {:?}, map: {:?} }}", self.data, self.map)
		}
	}
}

impl Collection for TypedObject {
	fn len(&self) -> usize {
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
}

impl Mapping for TypedObject {
	fn get(&self, key: &Object) -> Option<Object> {
		self.map.get(key)
	}

	#[inline]
	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.map.set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Object) -> Option<Object> {
		self.map.del(key)
	}

	#[inline]
	fn has(&self, key: &Object) -> bool {
		self.map.has(key)
	}
}


