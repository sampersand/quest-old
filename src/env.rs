use crate::{Shared, Object};
use crate::collections::{Collection, Mapping, Listing};
use std::fmt::{self, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Environment {
	parent: Option<Shared<Environment>>,
	map: Shared<dyn Mapping>,
	stack: Shared<dyn Listing>
}

lazy_static! {
	static ref CURRENT: Shared<Environment> = Shared::new(Environment {
		parent: None,
		map: Shared::new(crate::collections::Map::empty()),
		stack: Shared::new(crate::collections::List::empty())
	});
}

impl Environment {
	pub fn current() -> Shared<Environment> {
		CURRENT.clone()
	}
	// pub fn push_env(env: Shared<Environment>) ->
}

impl Display for Environment {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<environment; todo: this>")
	}
}


impl Collection for Environment {
	fn len(&self) -> usize {
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
}

impl Mapping for Environment {
	fn get(&self, key: &Object) -> Option<Object> {
		self.map.get(key)
	}

	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.map.set(key, val)
	}

	fn del(&mut self, key: &Object) -> Option<Object> {
		self.map.del(key)
	}

	fn has(&self, key: &Object) -> bool {
		self.map.has(key)
	}
}