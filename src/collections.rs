mod parental;
mod map;
mod list;

pub use self::{
	parental::ParentalMap,
	map::Map,
	list::List
};

use crate::{Shared, Object};
use std::fmt::Debug;
use mopa::mopafy;

pub trait Collection : mopa::Any + Debug + Send + Sync {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}


pub trait Mapping : Collection {
	fn get(&self, key: &Shared<Object>) -> Option<Shared<Object>>;
	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>>;
	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>>;
	fn has(&self, key: &Shared<Object>) -> bool;
}

mopafy!(Mapping);


pub trait Listing : Collection {
	fn push(&mut self, obj: Shared<Object>);
	fn pop(&mut self) -> Option<Shared<Object>>;
}

impl<T: Collection + ?Sized> Collection for Shared<T> {
	fn len(&self) -> usize {
		self.read().len()
	}

	fn is_empty(&self) -> bool {
		self.read().is_empty()
	}
}

impl<T: Mapping + ?Sized> Mapping for Shared<T> {
	fn get(&self, key: &Shared<Object>) -> Option<Shared<Object>> {
		self.read().get(key)
	}

	#[inline]
	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>> {
		self.write().set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>> {
		self.write().del(key)
	}

	#[inline]
	fn has(&self, key: &Shared<Object>) -> bool {
		self.read().has(key)
	}
}

