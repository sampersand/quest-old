use std::collections::HashMap;
use crate::{Shared, Object};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Map {
	data: HashMap<Shared<Object>, Shared<Object>>
}

impl Map {
	#[inline]
	pub fn new(data: HashMap<Shared<Object>, Shared<Object>>) -> Map {
		Map { data }
	}

	#[inline]
	pub fn empty() -> Map {
		Map::default()
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}


#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct List {
	objs: Vec<Shared<Object>>
}