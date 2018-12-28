mod typed;

pub use self::typed::TypedObject;

use crate::{Shared, Environment};
use crate::collections::{Collection, Mapping};

use std::any::TypeId;
use std::fmt::{self, Debug, Formatter};

pub struct Object {
	id: usize,
	mapid: TypeId,
	map: Shared<dyn Mapping>,
	env: Shared<Environment>
}

impl Object {
	pub fn new<M: Mapping + 'static>(map: M) -> Self {
		use std::sync::atomic::{AtomicUsize, Ordering};
		lazy_static::lazy_static! {
			static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		}

		Object {
			id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
			mapid: TypeId::of::<M>(),
			map: Shared::new(map) as _,
			env: Environment::current()
		}
	}
}

impl Eq for Object {}
impl PartialEq for Object {
	fn eq(&self, other: &Object) -> bool {
		unimplemented!()
		// let eq = self.map.read().eq();
		// (eq)(self, other)
	}
}

impl Clone for Object {
	fn clone(&self) -> Self {
		unimplemented!()
	}
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Object")
		 .field("id", &self.id)
		 .field("map", &self.map)
		 .field("env", &self.env)
		 .finish()
	}
}

impl Collection for Object {
	fn len(&self) -> usize {
		self.map.read().len()
	}

	fn is_empty(&self) -> bool {
		self.map.read().is_empty()
	}
}

impl Mapping for Object {
	fn get(&self, key: &Shared<Object>) -> Option<Shared<Object>> {
		self.map.read().get(key)
	}

	#[inline]
	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>> {
		self.map.write().set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>> {
		self.map.write().del(key)
	}

	#[inline]
	fn has(&self, key: &Shared<Object>) -> bool {
		self.map.read().has(key)
	}
}

