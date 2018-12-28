use crate::collections::{Collection, Listing};

use crate::SharedObject;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct List(Vec<SharedObject>);

impl List {
	#[inline]
	pub fn new(data: Vec<SharedObject>) -> List {
		List(data)
	}

	#[inline]
	pub fn empty() -> List {
		List::default()
	}

	pub fn iter(&self) -> impl Iterator<Item=&SharedObject> {
		self.0.iter()
	}
}

impl Collection for List {
	fn len(&self) -> usize {
		self.0.len()
	}

	fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl Listing for List {
	fn push(&mut self, obj: SharedObject) {
		self.0.push(obj)
	}

	fn pop(&mut self) -> Option<SharedObject> {
		self.0.pop()
	}
}

impl FromIterator<SharedObject> for List {
	fn from_iter<T: IntoIterator<Item=SharedObject>>(iter: T) -> List {
		List::new(Vec::from_iter(iter))
	}
}