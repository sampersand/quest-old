use crate::Shared;
use crate::collections::{Collection, Mapping};
use crate::object::{Object, r#type::Map as ObjMap};

#[derive(Debug, Clone)]
pub struct ParentalMap<M: Mapping = ObjMap> {
	parent: Shared<Object>,
	map: M
}

impl<M: Mapping + Default> ParentalMap<M> {
	pub fn new(parent: Shared<Object>) -> ParentalMap<M> {
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
		} else {
			self.parent.read().a
		}
		// self.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
	}

	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>> {
		for i in 0..self.data.len() {
			if self.data[i].0 == key {
				let v = self.data[i].1.clone();
				self.data[i] = (key, val);
				return Some(v)
			}
		}
		self.data.push((key, val));
		None
	}

	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>> {
		for i in 0..self.data.len() {
			if &self.data[i].0 == key {
				return Some(self.data.swap_remove(i).1)
			}
		}
		None
	}

	fn has(&self, key: &Shared<Object>) -> bool {
		self.keys().any(|k| k == key)
	}
}
