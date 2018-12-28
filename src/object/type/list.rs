use crate::{Shared, Mapping};
use crate::object::{Object, Type, IntoObject, r#type::Map};
use lazy_static::lazy_static;
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

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn push(&mut self, obj: Shared<Object>) {
		self.0.push(obj)
	}

	pub fn pop(&mut self) -> Option<Shared<Object>> {
		self.0.pop()
	}

}

impl FromIterator<Shared<Object>> for List {
	fn from_iter<T: IntoIterator<Item=Shared<Object>>>(iter: T) -> List {
		List::new(Vec::from_iter(iter))
	}
}


impl Type for List {
	fn create_map() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref CLASS: Shared<Object> = {
				let mut m = Map::empty();
				// m.set("+", unimplemented!())
				m
			}.into_shared();
		}

		Shared::new({
			let mut m = Map::empty();
			m.set("@parent".into_shared(), CLASS.clone());
			m
		}) as _
	}
}
