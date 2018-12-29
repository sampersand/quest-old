use crate::collections::{Collection, Mapping, Map};
use crate::SharedObject;

#[derive(Debug, Clone)]
pub struct ParentalMap<M: Mapping = Map> {
	parent: SharedObject,
	map: M
}

impl<M: Mapping + Default> ParentalMap<M> {
	pub fn new(parent: SharedObject) -> ParentalMap<M> {
		ParentalMap::new_mapped(parent, M::default())
	}
}

impl ParentalMap {
	pub fn new_default(parent: SharedObject) -> ParentalMap {
		ParentalMap::new_mapped(parent, Map::default())
	}
}

impl<M: Mapping> ParentalMap<M> {
	pub fn new_mapped(parent: SharedObject, map: M) -> ParentalMap<M> {
		ParentalMap { parent, map }
	}

}


impl<M: Mapping> Collection for ParentalMap<M> {
	fn len(&self) -> usize {
		// don't count parent in the size, as otherwise it's impossible to be empty
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		// don't count parent in the size, as otherwise it's impossible to be empty
		self.map.is_empty()
	}
}

impl<M: Mapping> Mapping for ParentalMap<M> {
	fn get(&self, key: &SharedObject) -> Option<SharedObject> {
		self.map.get(key).clone().or_else(|| self.parent.get(key))
	}

	#[inline]
	fn set(&mut self, key: SharedObject, val: SharedObject) -> Option<SharedObject> {
		self.map.set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &SharedObject) -> Option<SharedObject> {
		self.map.del(key)
	}

	#[inline]
	fn has(&self, key: &SharedObject) -> bool {
		self.map.has(key) || self.parent.has(key)
	}
}
