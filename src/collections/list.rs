use crate::collections::{Collection, Listing};

use crate::Object;
use std::iter::FromIterator;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct List(Vec<Object>);

impl List {
	#[inline]
	pub fn new(data: Vec<Object>) -> List {
		List(data)
	}

	#[inline]
	pub fn empty() -> List {
		List::default()
	}

	pub fn iter(&self) -> impl Iterator<Item=&Object> {
		self.0.iter()
	}
}

impl Display for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_list().entries(self.iter()).finish()
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
	fn push(&mut self, obj: Object) {
		self.0.push(obj)
	}

	fn pop(&mut self) -> Option<Object> {
		self.0.pop()
	}
}

impl FromIterator<Object> for List {
	fn from_iter<T: IntoIterator<Item=Object>>(iter: T) -> List {
		List::new(Vec::from_iter(iter))
	}
}