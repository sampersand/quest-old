#[macro_use]
mod macros;

mod basic;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


#[derive(Debug, Clone)]//, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Display for TypedObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.data {
			Types::Null => Display::fmt(&self::null::Null, f),
			Types::Bool(ref bool) => Display::fmt(bool, f),
			Types::Num(ref num) => Display::fmt(num, f),
			Types::Text(ref text) => Display::fmt(text, f),
			Types::Var(ref var) => Display::fmt(var, f),
			Types::RustFn(ref rustfn) => Display::fmt(rustfn, f),
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


