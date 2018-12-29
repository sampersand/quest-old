mod parental;
mod map;
mod list;

pub use self::{
	parental::ParentalMap,
	map::Map,
	list::List
};

use crate::{Shared, Object};
use std::fmt::{Debug, Display};
use mopa::mopafy;

pub trait Collection : mopa::Any + Debug + Display + Send + Sync {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}


pub trait Mapping : Collection {
	fn get(&self, key: &Object) -> Option<Object>;
	fn set(&mut self, key: Object, val: Object) -> Option<Object>;
	fn del(&mut self, key: &Object) -> Option<Object>;
	fn has(&self, key: &Object) -> bool;
}

mopafy!(Mapping);


pub trait Listing : Collection {
	fn push(&mut self, obj: Object);
	fn pop(&mut self) -> Option<Object>;
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
	fn get(&self, key: &Object) -> Option<Object> {
		self.read().get(key)
	}

	#[inline]
	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.write().set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Object) -> Option<Object> {
		self.write().del(key)
	}

	#[inline]
	fn has(&self, key: &Object) -> bool {
		self.read().has(key)
	}
}

