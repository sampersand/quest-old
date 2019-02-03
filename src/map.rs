mod parentmap;
pub use self::parentmap::ParentMap;

use std::fmt::Debug;
use crate::object::AnyObject;

pub trait Map : Debug + Send + Sync {
	#[must_use]
	fn get(&self, key: &AnyObject) -> Option<AnyObject>;
	fn set(&mut self, key: AnyObject, val: AnyObject) -> Option<AnyObject>; // we don't care what is returned
	fn del(&mut self, key: &AnyObject) -> Option<AnyObject>;
	fn len(&self) -> usize;

	#[inline]
	#[must_use]
	fn has(&self, key: &AnyObject) -> bool {
		self.get(key).is_some()
	}

	#[inline]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

use std::collections::HashMap;

impl Map for HashMap<AnyObject, AnyObject> {
	#[inline]
	fn get(&self, key: &AnyObject) -> Option<AnyObject> {
		HashMap::get(self, key).cloned()
	}

	#[inline]
	fn set(&mut self, key: AnyObject, val: AnyObject) -> Option<AnyObject> {
		self.insert(key, val)
	}

	#[inline]
	fn del(&mut self, key: &AnyObject) -> Option<AnyObject> {
		self.remove(key)
	}

	#[inline]
	fn has(&self, key: &AnyObject) -> bool {
		self.contains_key(key)
	}

	#[inline]
	fn len(&self) -> usize {
		self.len()
	}
}