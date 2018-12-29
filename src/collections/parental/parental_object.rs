use crate::{Shared, Object};
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
		ParentalObject {
			inner: RwLock::new(None),
			func: parent
		}
	}

	pub fn get(&self, key: &Object) -> Option<Object> {
		let inner = self.inner.read().expect("poisoned parental object read");
		if let Some(ref map) = *inner {
			map.get(key)
		} else {
			drop(inner);
			let mut inner = self.inner.write().expect("poisoned parental object write");
			if inner.is_none() { // in case it was created after we reacquired
				*inner = Some((self.func)());
			}
			inner.as_ref().unwrap().get(key)
		}
	}

	pub fn has(&self, key: &Object) -> bool {
		let inner = self.inner.read().expect("poisoned parental object read");
		if let Some(ref map) = *inner {
			map.has(key)
		} else {
			drop(inner);
			let mut inner = self.inner.write().expect("poisoned parental object write");
			if inner.is_none() { // in case it was created after we reacquired
				*inner = Some((self.func)());
			}
			inner.as_ref().unwrap().has(key)
		}
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

