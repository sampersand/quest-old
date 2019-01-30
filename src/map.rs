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