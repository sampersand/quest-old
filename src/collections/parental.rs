use crate::Object;
use crate::collections::{Collection, Mapping, Map};
use std::sync::{Mutex, Once};
use std::fmt::{self, Debug, Display, Formatter};

mod parental_object;
use self::parental_object::{ParentalObject, InitFunc};

#[derive(Clone)]
pub struct ParentalMap<M: Mapping = Map> {
	parent: ParentalObject,
	map: M
}

impl<M: Mapping + Default> ParentalMap<M> {
	pub fn new(parent: InitFunc) -> ParentalMap<M> {
		ParentalMap::new_mapped(parent, M::default())
	}
}

impl ParentalMap {
	pub fn new_default(parent: InitFunc) -> ParentalMap {
		ParentalMap::new_mapped(parent, Map::default())
	}
}

impl<M: Mapping> ParentalMap<M> {
	pub fn new_mapped(parent: InitFunc, map: M) -> ParentalMap<M> {
		ParentalMap { parent: ParentalObject::new(parent), map }
	}

}

impl<M: Mapping> Debug for ParentalMap<M> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("ParentalMap")
			 .field("parent", &self.parent)
			 .field("map", &self.map)
			 .finish()
		} else {
			write!(f, "P")?;
			Debug::fmt(&self.map, f)
		}
	}
}

impl<M: Mapping> Display for ParentalMap<M> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// maybe do something with `parent` here?
		Display::fmt(&self.map, f)
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
	fn get(&self, key: &Object) -> Option<Object> {
		if let Some(var) = key.downcast_var() {
			if *var.as_ref() == "@parent" {
				return Some(self.parent.inner())
			}
		}
		self.map.get(key).clone().or_else(|| self.parent.get(key))
	}

	#[inline]
	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		if let Some(var) = key.downcast_var() {
			if *var.as_ref() == "@parent" {
				let old_parent = self.parent.inner();
				self.parent = ParentalObject::new_initialized(val);
				return Some(old_parent)
			}
		}
		self.map.set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Object) -> Option<Object> {
		if let Some(var) = key.downcast_var() {
			if *var.as_ref() == "@parent" {
				warn!("Cannot delete @parent off of ParentalMap");
				return None;
			}
		}
		self.map.del(key)
	}

	#[inline]
	fn has(&self, key: &Object) -> bool {
		if let Some(var) = key.downcast_var() {
			if *var.as_ref() == "@parent" {
				return true;
			}
		}

		self.map.has(key) || self.parent.has(key)
	}
}
