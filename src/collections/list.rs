use crate::collections::{Collection, Listing};

use crate::{Shared, Object};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct List(Vec<Shared<Object>>);

impl List {
	#[inline]
	pub fn new(data: Vec<Shared<Object>>) -> List {
		List(data)
	}

	#[inline]
	pub fn empty() -> List {
		List::default()
	}

	pub fn iter(&self) -> impl Iterator<Item=&Shared<Object>> {
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
	fn push(&mut self, obj: Shared<Object>) {
		self.0.push(obj)
	}

	fn pop(&mut self) -> Option<Shared<Object>> {
		self.0.pop()
	}
}

impl FromIterator<Shared<Object>> for List {
	fn from_iter<T: IntoIterator<Item=Shared<Object>>>(iter: T) -> List {
		List::new(Vec::from_iter(iter))
	}
}