use std::sync::{Arc, RwLock};
use std::hash::{Hash, Hasher};

#[derive(Debug, Default)]
pub struct Shared<T: ?Sized> {
	data: Arc<RwLock<T>>
}


impl<T> Shared<T> {
	pub fn new(data: T) -> Shared<T> {
		Shared {
			data: Arc::new(RwLock::new(data))
		}
	}
}
impl<T: ?Sized> Shared<T> {

	pub fn ptr_eq(&self, other: &Shared<T>) -> bool {
		Arc::ptr_eq(&self.data, &other.data)
	}
}

impl<T> Clone for Shared<T> {
	fn clone(&self) -> Shared<T> {
		Shared { data: self.data.clone() }
	}
}

impl<T: Eq + ?Sized> Eq for Shared<T> {}
impl<T: PartialEq + ?Sized> PartialEq for Shared<T> {
	fn eq(&self, other: &Shared<T>) -> bool {
		if self.ptr_eq(other) {
			true
		} else {
			self.data.read().expect("Bad read").eq(&other.data.read().expect("Bad read"))
		}
	}
}

impl<T: Hash + ?Sized> Hash for Shared<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.data.read().expect("Bad read").hash(h)
	}
}