use std::sync::{Arc, RwLock, LockResult};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Shared<T: ?Sized>(Arc<RwLock<T>>);

impl<T: ?Sized> Clone for Shared<T> {
	fn clone(&self) -> Shared<T> {
		Shared(self.0.clone())
	}
}

impl<T> Shared<T> {
	pub fn new(item: T) -> Shared<T> {
		Shared(Arc::new(RwLock::new(item)))
	}
}

impl<T: ?Sized> Shared<T> {
	pub fn read(&self) -> LockResult<impl Deref<Target=T> + '_> {
		self.0.read()
	}

	pub fn write(&self) -> LockResult<impl DerefMut<Target=T> + '_> {
		self.0.write()
	}
}