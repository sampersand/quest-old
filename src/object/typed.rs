#[macro_use]
mod macros;

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
		Shared::new(crate::object::ObjectInner::new(self))
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


