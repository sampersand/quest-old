use std::fmt::Debug;
use crate::object::AnyObject;

pub trait Map : Debug + Send + Sync {
	fn get(&self, key: &AnyObject) -> Option<AnyObject>;
	fn set(&mut self, key: AnyObject, val: AnyObject); // we don't care what is returned
	fn del(&mut self, key: &AnyObject) -> Option<AnyObject>;

	#[inline]
	fn has(&self, key: &AnyObject) -> bool {
		self.get(key).is_some()
	}
}

use std::collections::HashMap;

impl Map for HashMap<AnyObject, AnyObject> {
	#[inline]
	fn get(&self, key: &AnyObject) -> Option<AnyObject> {
		HashMap::get(self, key).cloned()
	}

	#[inline]
	fn set(&mut self, key: AnyObject, val: AnyObject) {
		self.insert(key, val);
	}

	#[inline]
	fn del(&mut self, key: &AnyObject) -> Option<AnyObject> {
		self.remove(key)
	}

	#[inline]
	fn has(&self, key: &AnyObject) -> bool {
		self.contains_key(key)
	}
}