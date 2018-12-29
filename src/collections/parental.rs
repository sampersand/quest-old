use crate::collections::{Collection, Mapping, Map};
use crate::{Shared, Object};
use std::sync::Once;

#[derive(Debug)]
struct ParentalObject {
	inner: Shared<Object>,
	init: Once,
	func: fn() -> Shared<Object>
}

#[derive(Debug, Clone)]
pub struct ParentalMap<M: Mapping = Map> {
	parent: Shared<Object>,
	map: M
}

impl<M: Mapping + Default> ParentalMap<M> {
	pub fn new(parent: Shared<Object>) -> ParentalMap<M> {
		ParentalMap::new_mapped(parent, M::default())
	}
}

impl ParentalMap {
	pub fn new_default(parent: Shared<Object>) -> ParentalMap {
		ParentalMap::new_mapped(parent, Map::default())
	}
}

impl<M: Mapping> ParentalMap<M> {
	pub fn new_mapped(parent: Shared<Object>, map: M) -> ParentalMap<M> {
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
	fn get(&self, key: &Shared<Object>) -> Option<Shared<Object>> {
		self.map.get(key).clone().or_else(|| self.parent.get(key))
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
		self.map.has(key) || self.parent.has(key)
	}
}
