use crate::Shared;
use crate::collections::{Collection, Mapping};
use crate::object::{Object, r#type::Map as ObjMap};

#[derive(Debug, Clone)]
pub struct ParentalMap<M: Mapping = ObjMap> {
	parent: Shared<dyn Mapping>,
	map: M
}

impl<M: Mapping + Default> ParentalMap<M> {
	pub fn new(parent: Shared<dyn Mapping>) -> ParentalMap<M> {
		unimplemented!()
	}
}


impl Collection for ParentalMap {
	fn len(&self) -> usize {
		// don't count parent in the size, as otherwise it's impossible to be empty
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		// don't count parent in the size, as otherwise it's impossible to be empty
		self.map.is_empty()
	}
}

impl Mapping for ParentalMap {
	fn get(&self, key: &Shared<Object>) -> Option<&Shared<Object>> {
		if let Some(obj) = self.map.get(key) {
			Some(obj)
			// todo: make object a map
		} else {
			self.parent.read().get(key)
		}
		// self.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
	}

	#[inline]
	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>> {
		self.map.set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>> {
		self.map.del(key)
	}

	#[inline]
	fn has(&self, key: &Shared<Object>) -> bool {
		self.map.has(key) || self.parent.read().has(key)
	}
}
