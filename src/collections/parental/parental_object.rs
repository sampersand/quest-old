use crate::Object;
use crate::collections::Mapping;
use std::sync::RwLock;
use std::fmt::{self, Debug, Formatter};

pub type InitFunc = fn() -> Object;

pub struct ParentalObject {
	// this rwlock can be replaced later for efficiency
	inner: RwLock<Option<Object>>,
	func: InitFunc
}

impl Clone for ParentalObject {
	fn clone(&self) -> ParentalObject {
		ParentalObject {
			inner: RwLock::new(self.inner.try_read().expect("shouldn't be cloning whilst locked").clone()),
			func: self.func
		}
	}
}

impl ParentalObject {
	pub fn new(parent: InitFunc) -> ParentalObject {
		trace!("Creating a new uninitialized ParentalObject for {:p}", parent as *const ());
		ParentalObject {
			inner: RwLock::new(None),
			func: parent
		}
	}

	pub fn new_initialized(parent: Object) -> ParentalObject {
		trace!("Creating a new initialized ParentalObject for {:?}", parent);
		ParentalObject {
			inner: RwLock::new(Some(parent)),
			func: || unreachable!("Attempted to create a parent that already exists")
		}
	}

	fn get_parent<T, F: Fn(&Object) -> T>(&self, func: F) -> T {
		let inner = self.inner.read().expect("poisoned parental object read");
		if let Some(ref map) = *inner {
			func(map)
		} else {
			drop(inner);
			let mut inner = self.inner.write().expect("poisoned parental object write");
			if inner.is_none() { // in case it was created after we reacquired
				debug!("Initialized ParentalObject {:p}", self as *const _);
				*inner = Some((self.func)());
			}
			func(inner.as_ref().unwrap())
		}
	}

	pub fn get(&self, key: &Object) -> Option<Object> {
		self.get_parent(|map| map.get(key))
	}

	pub fn has(&self, key: &Object) -> bool {
		self.get_parent(|map| map.has(key))
	}

	pub fn inner(&self) -> Object {
		self.get_parent(|map| map.clone())
	}
}


impl Debug for ParentalObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if let Some(ref map) = *self.inner.read().expect("poisoned parental object read") {
			Debug::fmt(map, f)
		} else {
			write!(f, "<unitialized map>")
		}
	}
}

