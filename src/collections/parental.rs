use crate::collections::{Collection, Mapping};
use crate::object::{Object, r#type::Map as ObjMap};

#[derive(Debug, Clone)]
pub struct ParentalMap<M: Mapping = ObjMap> {
	parent: Object,
	map: M
}

impl<M: Mapping + Default> ParentalMap<M> {
	pub fn new(parent: Object) -> ParentalMap<M> {
		ParentalMap::new_mapped(parent, M::default())
	}
}

impl<M: Mapping> ParentalMap<M> {
	pub fn new_mapped(parent: Object, map: M) -> ParentalMap<M> {
		ParentalMap { parent, map }
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
	fn get(&self, key: &Object) -> Option<Object> {
		self.map.get(key).clone().or_else(|| self.parent.read().get(key))
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
		self.map.has(key) || self.parent.read().has(key)
	}
}
