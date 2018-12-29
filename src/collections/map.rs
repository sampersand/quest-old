use crate::Object;
use crate::collections::{Collection, Mapping};
use std::iter::FromIterator;
use std::fmt::{self, Debug, Display, Formatter};

type Pair = (Object, Object);

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Map {
	data: Vec<Pair>
}

impl Map {
	#[inline]
	pub fn new(data: Vec<Pair>) -> Map {
		Map { data }
	}

	#[inline]
	pub fn empty() -> Map {
		Map::default()
	}

	pub fn iter(&self) -> impl Iterator<Item=&Pair> {
		self.data.iter()
	}

	pub fn keys(&self) -> impl Iterator<Item=&Object> {
		struct KeyIter<'a>(std::slice::Iter<'a, Pair>);
		impl<'a> Iterator for KeyIter<'a> {
			type Item = &'a Object;
			fn next(&mut self) -> Option<&'a Object> {
				self.0.next().map(|(k, _)| k)
			}
		}
		KeyIter(self.data.iter())
	}
}

impl Debug for Map {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_map().entries(self.data.iter().map(Clone::clone)).finish()
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self, f)
	}
}

impl Collection for Map {
	fn len(&self) -> usize {
		self.data.len()
	}

	fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}

impl Mapping for Map {
	fn get(&self, key: &Object) -> Option<Object> {
		self.iter().find_map(|(k, v)| if k == key { Some(v) } else { None }).cloned()
	}

	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
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

	fn del(&mut self, key: &Object) -> Option<Object> {
		for i in 0..self.data.len() {
			if &self.data[i].0 == key {
				return Some(self.data.swap_remove(i).1)
			}
		}
		None
	}

	fn has(&self, key: &Object) -> bool {
		self.keys().any(|k| k == key)
	}
}

impl FromIterator<Pair> for Map {
	fn from_iter<T: IntoIterator<Item=Pair>>(iter: T) -> Map {
		Map::new(Vec::from_iter(iter))
	}
}